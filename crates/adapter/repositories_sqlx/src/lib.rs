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

