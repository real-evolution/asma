use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::BoxStream;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppResult;

#[async_trait]
pub trait MessagePassingService: Send + Sync {
    async fn get_topic_writer<T>(
        &self,
        name: &str,
    ) -> AppResult<Arc<dyn TopicWriter<T>>>
    where
        T: Serialize + Send + Sync + 'static;

    async fn get_topic_reader<T>(
        &self,
        name: &str,
    ) -> AppResult<Arc<dyn TopicReader<T>>>
    where
        T: DeserializeOwned + Send + Sync + 'static;
}

#[async_trait]
pub trait MessageConfirmation: Send + Sync + 'static {
    async fn ack(&self) -> AppResult<()>;
    async fn nack(&self, requeue: bool) -> AppResult<()>;
}

#[async_trait]
pub trait TopicWriter<T>: Send + Sync {
    async fn publish(&self, key: &str, body: &T) -> AppResult<()>;
    async fn publish_confirmed(&self, key: &str, body: &T) -> AppResult<()>;

    fn scoped(&self, key: &str) -> Arc<dyn ScopedTopicWriter<T>>;
}

#[async_trait]
pub trait ScopedTopicWriter<T>: Send + Sync {
    async fn publish(&self, body: &T) -> AppResult<()>;
    async fn publish_confirmed(&self, body: &T) -> AppResult<()>;
}

#[async_trait]
pub trait TopicReader<T>: Send + Sync {
    async fn subscribe(
        &self,
        key: &str,
    ) -> AppResult<BoxStream<'_, AppResult<T>>>;

    async fn subscribe_manual(
        &self,
        key: &str,
    ) -> AppResult<BoxStream<'_, AppResult<(T, Arc<dyn MessageConfirmation>)>>>;

    async fn mirror(&self, key: &str)
        -> AppResult<BoxStream<'_, AppResult<T>>>;

    fn scoped(&self, key: &str) -> Arc<dyn ScopedTopicReader<T>>;
}

#[async_trait]
pub trait ScopedTopicReader<T>: Send + Sync {
    async fn subscribe(&self) -> AppResult<BoxStream<'_, AppResult<T>>>;

    async fn subscribe_manual(
        &self,
    ) -> AppResult<BoxStream<'_, AppResult<(T, Arc<dyn MessageConfirmation>)>>>;

    async fn mirror(&self) -> AppResult<BoxStream<'_, AppResult<T>>>;
}

#[async_trait]
pub trait Topic<T>: TopicReader<T> + TopicWriter<T> {}

impl<T, U> Topic<T> for U where U: TopicWriter<T> + TopicReader<T> {}
