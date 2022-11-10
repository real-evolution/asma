use async_trait::async_trait;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait]
pub trait TransactionManager: Interface {
    async fn begin(&self) -> RepoResult<Box<dyn Transaction>>;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    async fn commit(self: Box<Self>) -> RepoResult<()>;
    async fn rollback(self: Box<Self>) -> RepoResult<()>;
}
