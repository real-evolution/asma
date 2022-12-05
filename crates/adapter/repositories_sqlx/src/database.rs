use async_trait::async_trait;
use kernel_repositories::{error::RepoResult, Transaction, TransactionManager};
use sqlx::pool::PoolConnection;

use crate::util::error::map_sqlx_error;

pub(crate) type DbType = sqlx::postgres::Postgres;
pub(crate) type PoolType = sqlx::Pool<DbType>;

#[derive(Debug, Clone)]
pub(crate) struct SqlxPool(pub PoolType);

#[derive(Debug)]
struct SqlxTransactionWrapper<'c>(sqlx::Transaction<'c, DbType>);

impl SqlxPool {
    pub(crate) fn get(&self) -> &PoolType {
        &self.0
    }

    pub(crate) async fn acquire(&self) -> RepoResult<PoolConnection<DbType>> {
        Ok(self.0.acquire().await.map_err(map_sqlx_error)?)
    }
}

#[async_trait]
impl TransactionManager for SqlxPool {
    async fn begin(&self) -> RepoResult<Box<dyn Transaction>> {
        let tx = self.0.begin().await.map_err(map_sqlx_error)?;

        Ok(Box::new(SqlxTransactionWrapper(tx)))
    }
}

#[async_trait]
impl<'c> Transaction for SqlxTransactionWrapper<'c> {
    async fn commit(self: Box<Self>) -> RepoResult<()> {
        Ok(self.0.commit().await.map_err(map_sqlx_error)?)
    }

    async fn rollback(self: Box<Self>) -> RepoResult<()> {
        Ok(self.0.rollback().await.map_err(map_sqlx_error)?)
    }
}
