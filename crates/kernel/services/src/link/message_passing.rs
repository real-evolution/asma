use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::BoxStream;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppResult;

#[async_trait]
pub trait MessagePassingService: Send + Sync {
    async fn publish_raw(
        &self,
        topic: &str,
        key: Option<&str>,
        body: &[u8],
    ) -> AppResult<()>;

    async fn publish<T: Serialize + Send + Sync>(
        &self,
        topic: &str,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()>;

    fn subscribe<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> BoxStream<'a, AppResult<T>>;

    fn subscribe_manual<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> BoxStream<'a, AppResult<(T, Arc<dyn MessageConfirmation>)>>;
}

#[async_trait]
pub trait MessageConfirmation: Send + Sync + 'static {
    async fn ack(self) -> AppResult<()>;
    async fn nack(self, requeue: bool) -> AppResult<()>;
}
