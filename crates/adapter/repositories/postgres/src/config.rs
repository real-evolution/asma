use std::time::Duration;

use common_validation::*;
use kernel_repositories::error::{RepoError, RepoResult};
use serde::Deserialize;
use validator::Validate;

use crate::util::error::map_sqlx_error;

pub const DATA_CONFIG_SECTION: &str = "data";

#[derive(Debug, Deserialize, Validate)]
pub struct DataConfig {
    #[validate(custom = "supported_data_driver")]
    pub driver: String,
    #[validate(custom = "endpoint")]
    pub host: String,
    #[validate(range(min = 0, max = 0xFFFF))]
    pub port: Option<u16>,
    #[validate(custom = "identifier")]
    pub username: String,
    pub password: String,
    #[validate(custom = "identifier")]
    pub database: String,
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
    pub lazy: Option<bool>,
}

impl DataConfig {
    pub fn get_connection_string(&self) -> RepoResult<String> {
        self.do_get_connection_string::<false>()
    }

    pub fn get_concealed_connection_string(&self) -> RepoResult<String> {
        self.do_get_connection_string::<true>()
    }

    pub async fn into_pool<Db: sqlx::Database>(self) -> RepoResult<sqlx::Pool<Db>> {
        let url = self.get_connection_string()?;
        let mut opts = sqlx::pool::PoolOptions::<Db>::new();

        if let Some(min) = self.pool.min_connections {
            opts = opts.min_connections(min);
        }

        if let Some(max) = self.pool.max_connections {
            opts = opts.max_connections(max);
        }

        if let Some(max_lifetime) = self.pool.max_lifetime_ms {
            opts = opts.max_lifetime(Duration::from_millis(max_lifetime));
        }

        if let Some(idle_timeout) = self.pool.idle_timeout_ms {
            opts = opts.idle_timeout(Duration::from_millis(idle_timeout));
        }

        let pool = if self.pool.lazy.unwrap_or(false) {
            opts.connect_lazy(&url).map_err(map_sqlx_error)?
        } else {
            opts.connect(&url).await.map_err(map_sqlx_error)?
        };

        Ok(pool)
    }

    pub fn do_get_connection_string<const CONCEALED: bool>(
        &self,
    ) -> RepoResult<String> {

        let ep = Endpoint::parse_str(&self.host)
            .map_err(|err| RepoError::InvalidParameter(err.to_string()))?;
        let host = match self.port.or(ep.port) {
            | Some(port) => format!("{}:{}", ep.domain, port),
            | None => ep.domain,
        };

        let password = if CONCEALED { "***" } else { &self.password };

        Ok(format!(
            "{}://{}:{}@{}/{}",
            self.driver, self.username, password, host, self.database
        ))
    }
}
