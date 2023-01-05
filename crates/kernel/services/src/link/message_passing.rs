use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::BoxStream;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppResult;

#[async_trait]
pub trait MessagePassingService: Send + Sync {
    #[deprecated]
    async fn get_topic<T>(&self, name: &str) -> AppResult<Arc<dyn Topic<T>>>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static;
}

#[async_trait]
pub trait MessageConfirmation: Send + Sync + 'static {
    async fn ack(&self) -> AppResult<()>;
    async fn nack(&self, requeue: bool) -> AppResult<()>;
}

#[async_trait]
pub trait TopicWriter<T>: Send + Sync {
    async fn publish(&self, key: Option<&str>, body: &T) -> AppResult<()>;

    async fn publish_confirmed(
        &self,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()>;
}

#[async_trait]
pub trait TopicReader<T>: Send + Sync {
    async fn subscribe(
        &self,
        key: Option<&str>,
    ) -> AppResult<BoxStream<'_, AppResult<T>>>;

    async fn subscribe_manual(
        &self,
        key: Option<&str>,
    ) -> AppResult<BoxStream<'_, AppResult<(T, Arc<dyn MessageConfirmation>)>>>;

    async fn mirror(
        &self,
        key: Option<&str>,
    ) -> AppResult<BoxStream<'_, AppResult<T>>>;
}

#[async_trait]
pub trait Topic<T>: TopicReader<T> + TopicWriter<T> {}

impl<T, U> Topic<T> for U where U: TopicWriter<T> + TopicReader<T>{}
