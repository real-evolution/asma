use std::sync::Arc;

use async_trait::async_trait;
use shaku::Interface;

use crate::RepoResult;

#[async_trait]
pub trait TransactionManager: Interface {
    async fn begin(&self) -> RepoResult<Arc<dyn Transaction>>;
}

#[async_trait]
pub trait Transaction {
    async fn commit(self) -> RepoResult<()>;
    async fn rollback(self) -> RepoResult<()>;
}
