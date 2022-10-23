use crate::DataConfig;

use shaku::{Component, Interface};
use std::time::Duration;

pub type DbType = sqlx::postgres::Postgres;
pub type PoolType = sqlx::Pool<DbType>;

#[derive(Component)]
#[shaku(interface = DbConnection)]
pub struct SqlxDatabase {
    pool: Option<PoolType>,
}

impl SqlxDatabase {
    pub fn new() -> Self {
        Self { pool: None }
    }

    pub async fn configure<'a>(
        &mut self,
        config: &'a DataConfig<'a>,
    ) -> anyhow::Result<()> {
        if self.is_open() {
            panic!("database connection is already open");
        }

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

        self.pool = Some(pool);

        Ok(())
    }
}

pub trait DbConnection: Interface {
    fn into_inner_ref(&self) -> &PoolType;
    fn is_open(&self) -> bool;
}

impl DbConnection for SqlxDatabase {
    fn into_inner_ref(&self) -> &PoolType {
        self.pool.as_ref().unwrap()
    }

    fn is_open(&self) -> bool {
        self.pool.is_some()
    }
}
