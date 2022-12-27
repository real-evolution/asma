use futures::stream::BoxStream;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppResult;

#[async_trait::async_trait]
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
}
