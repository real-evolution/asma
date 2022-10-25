use core::ops::Deref;
use shaku::{Component, Interface};

pub type DbType = sqlx::postgres::Postgres;
pub type PoolType = sqlx::Pool<DbType>;

#[derive(Component)]
#[shaku(interface = DatabaseConnection)]
pub struct SqlxDatabaseConnection {
    pool: PoolType,
}

#[async_trait::async_trait()]
pub trait DatabaseConnection: Interface + Deref<Target = PoolType> {
    fn is_closed(&self) -> bool;
}

#[async_trait::async_trait()]
impl DatabaseConnection for SqlxDatabaseConnection {
    fn is_closed(&self) -> bool {
        self.pool.is_closed()
    }
}

impl std::ops::Deref for SqlxDatabaseConnection {
    type Target = PoolType;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
