use async_trait::async_trait;

use crate::error::RepoResult;

#[async_trait]
pub trait TransactionManager: Send + Sync {
    async fn begin(&self) -> RepoResult<Box<dyn Transaction>>;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    async fn commit(self: Box<Self>) -> RepoResult<()>;
    async fn rollback(self: Box<Self>) -> RepoResult<()>;
}
