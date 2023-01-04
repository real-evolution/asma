use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::BoxStream;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppResult;

#[async_trait]
pub trait MessagePassingService: Send + Sync {
    type TopicType: Topic;

    async fn get_topic(&self, name: &str) -> AppResult<Arc<Self::TopicType>>;
}

#[async_trait]
pub trait MessageConfirmation: Send + Sync + 'static {
    async fn ack(self) -> AppResult<()>;
    async fn nack(self, requeue: bool) -> AppResult<()>;
}

#[async_trait]
pub trait Topic {
    async fn publish<T: Serialize + Send + Sync>(
        &self,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()>;

    async fn publish_confirmed<T: Serialize + Send + Sync>(
        &self,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()>;

    async fn subscribe<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        key: Option<&'a str>,
    ) -> AppResult<BoxStream<'a, AppResult<T>>>;

    async fn subscribe_manual<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        key: Option<&'a str>,
    ) -> AppResult<BoxStream<'a, AppResult<(T, Arc<dyn MessageConfirmation>)>>>;

    async fn mirror<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        key: Option<&'a str>,
    ) -> AppResult<BoxStream<'a, AppResult<T>>>;
}
