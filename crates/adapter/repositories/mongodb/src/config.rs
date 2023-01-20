use std::time::Duration;

use common_validation::*;
use kernel_repositories::error::{RepoError, RepoResult};
use mongodb::{options::ClientOptions, Client, Database};
use serde::Deserialize;
use validator::Validate;

use crate::util::error::map_mongo_error;

pub const DOC_STORE_CONFIG_SECTION: &str = "docs";

#[derive(Debug, Deserialize, Validate)]
pub struct DocumentStoreConfig {
    #[validate(custom = "supported_doc_store_driver")]
    pub driver: String,
    #[validate(custom = "endpoint")]
    pub host: String,
    #[validate(range(min = 0, max = 0xFFFF))]
    pub port: Option<u16>,
    #[validate(custom = "identifier")]
    pub database: String,
    #[validate(custom = "identifier")]
    pub repl_name: Option<String>,
    #[validate]
    pub pool: PoolConfig,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct PoolConfig {
    #[validate(range(min = 1, max = 64))]
    pub min_connections: Option<u32>,
    #[validate(range(min = 1, max = 64))]
    pub max_connections: Option<u32>,
    pub max_lifetime_ms: Option<u64>,
    pub idle_timeout_ms: Option<u64>,
}

impl DocumentStoreConfig {
    pub fn get_connection_string(&self) -> RepoResult<String> {
        if let Err(err) = self.validate() {
            return Err(RepoError::InvalidParameter(err.to_string()));
        }

        let ep = Endpoint::parse_str(&self.host)
            .map_err(|err| RepoError::InvalidParameter(err.to_string()))?;
        let host = match self.port.or(ep.port) {
            | Some(port) => format!("{}:{}", ep.domain, port),
            | None => ep.domain,
        };

        Ok(format!("{}://{}/{}", self.driver, host, self.database))
    }

    pub async fn into_client(self) -> RepoResult<(Client, Database)> {
        let url = self.get_connection_string()?;

        let mut opts =
            ClientOptions::parse(url).await.map_err(map_mongo_error)?;

        opts.app_name = Some("asma".to_owned());
        opts.max_pool_size = self.pool.max_connections;
        opts.min_pool_size = self.pool.min_connections;
        opts.repl_set_name = self.repl_name;
        opts.max_idle_time =
            self.pool.max_lifetime_ms.map(Duration::from_millis);

        let client = Client::with_options(opts).map_err(map_mongo_error)?;
        let database = client.database(&self.database);

        Ok((client, database))
    }
}
