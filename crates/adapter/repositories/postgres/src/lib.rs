use std::sync::Arc;

use auth::SqlxAuthDataStore;
use database::SqlxPool;
use kernel_repositories::{auth::AuthDataStore, link::LinkDataStore, *};
use link::SqlxLinkDataStore;

mod auth;
mod config;
mod database;
mod link;
mod util;

pub use config::*;

struct SqlxDataStore {
    pool: SqlxPool,
    auth: SqlxAuthDataStore,
    link: SqlxLinkDataStore,
}

impl DataStore for SqlxDataStore {
    fn tx(&self) -> &dyn TransactionManager {
        &self.pool
    }

    fn auth(&self) -> &dyn AuthDataStore {
        &self.auth
    }

    fn link(&self) -> &dyn LinkDataStore {
        &self.link
    }
}

pub async fn create_datastore(
    conf: config::DataConfig,
) -> anyhow::Result<Arc<dyn DataStore>> {
    tracing::debug!(
        "openning database connection to: {}",
        conf.get_concealed_connection_string()?
    );

    let pool = SqlxPool(conf.into_pool().await?);

    Ok(Arc::new(SqlxDataStore {
        auth: SqlxAuthDataStore::new(pool.clone()),
        link: SqlxLinkDataStore::new(pool.clone()),
        pool,
    }))
}
