use std::sync::Arc;

use auth::SqlxAuthDataStore;
use database::SqlxPool;
use kernel_repositories::{
    auth::AuthDataStore, comm::CommDataStore, link::LinkDataStore, *,
};
use link::SqlxLinkDataStore;

mod auth;
mod comm;
mod config;
mod database;
mod link;
mod util;

pub use config::*;

use crate::comm::SqlxCommDataStore;

struct SqlxDataStore {
    pool: SqlxPool,
    auth: SqlxAuthDataStore,
    link: SqlxLinkDataStore,
    comm: comm::SqlxCommDataStore,
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

    fn comm(&self) -> &dyn CommDataStore {
        &self.comm
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

    tracing::debug!("migrating database");
    pool.migrate().await?;

    Ok(Arc::new(SqlxDataStore {
        auth: SqlxAuthDataStore::new(pool.clone()),
        link: SqlxLinkDataStore::new(pool.clone()),
        comm: SqlxCommDataStore::new(pool.clone()),
        pool,
    }))
}
