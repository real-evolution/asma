use async_trait::async_trait;
use kernel_repositories::{TransactionManager, error::RepoResult, Transaction};
use shaku::{Component, Interface};

use crate::util::map_sqlx_error;


pub type DbType = sqlx::postgres::Postgres;
pub type PoolType = sqlx::Pool<DbType>;

pub trait SqlxDatabaseConnection: Interface {
    fn get(&self) -> &PoolType;
}

#[derive(Component)]
#[shaku(interface = SqlxDatabaseConnection)]
pub struct SqlxPool {
    inner: PoolType,
}

#[derive(Component)]
#[shaku(interface = TransactionManager)]
pub struct SqlxTransactionManager {
    inner: PoolType,
}

impl SqlxDatabaseConnection for SqlxPool {
    fn get(&self) -> &PoolType {
        &self.inner
    }
}

#[async_trait]
impl TransactionManager for SqlxTransactionManager {
    async fn begin(&self) -> RepoResult<Box<dyn Transaction>> {
        let tx = self.inner.begin().await.map_err(map_sqlx_error)?;

        Ok(Box::new(SqlxTransactionWrapper(tx)))
    }
}

struct SqlxTransactionWrapper<'c>(sqlx::Transaction<'c, DbType>);

#[async_trait]
impl<'c> Transaction for SqlxTransactionWrapper<'c> {
    async fn commit(self: Box<Self>) -> RepoResult<()> {
        Ok(self.0.commit().await.map_err(map_sqlx_error)?)
    }

    async fn rollback(self: Box<Self>) -> RepoResult<()> {
        Ok(self.0.rollback().await.map_err(map_sqlx_error)?)
    }
}
