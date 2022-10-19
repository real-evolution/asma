use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use kernel_entities::traits::BasicEntity;
use kernel_repositories::Repo;

use crate::DataConfig;

pub type DbType = sqlx::postgres::Postgres;

pub struct SqlxDatabase {
    pool: sqlx::Pool<DbType>,
}

impl SqlxDatabase {
    pub fn new(pool: sqlx::Pool<DbType>) -> Self {
        Self { pool }
    }

    pub async fn from_config<'a>(
        config: &'a DataConfig<'a>,
    ) -> anyhow::Result<Self> {
        let url = config.get_connection_string()?;
        let mut opts = sqlx::pool::PoolOptions::<DbType>::new();

        if let Some(min) = config.pool.min_connections {
            opts = opts.min_connections(min);
        }

        if let Some(max) = config.pool.max_connections {
            opts = opts.max_connections(max);
        }

        if let Some(max_lifetime) = config.pool.max_lifetime_ms {
            opts = opts.max_lifetime(Duration::from_millis(max_lifetime));
        }

        if let Some(idle_timeout) = config.pool.idle_timeout_ms {
            opts = opts.idle_timeout(Duration::from_millis(idle_timeout));
        }

        let pool = if config.pool.lazy.unwrap_or(false) {
            opts.connect_lazy(&url)?
        } else {
            opts.connect(&url).await?
        };

        Ok(Self::new(pool))
    }
}

impl<E, K> Repo<E, K> for SqlxDatabase where E: BasicEntity<Key = K> {}

impl Deref for SqlxDatabase {
    type Target = sqlx::Pool<DbType>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for SqlxDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}
