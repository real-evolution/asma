use std::{collections::HashMap, sync::Arc, time::Duration};

use async_stream::stream;
use deadpool_lapin::{Config, Object, Pool, Runtime};
use futures::{stream::BoxStream, StreamExt};
use kernel_services::error::AppResult;
use lapin::Channel;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;

use super::{
    config::MessageQueueConfig,
    topic::RabbitMQTopic,
    util::map_ipc_error,
};

pub(super) struct RabbitMQContext {
    config: MessageQueueConfig,
    pool: Pool,
    topics: RwLock<HashMap<String, Arc<RabbitMQTopic>>>,
}

impl RabbitMQContext {
    pub(super) async fn create(
        config: MessageQueueConfig,
    ) -> anyhow::Result<Self> {
        debug!(
            "openning rabbitmq connection to: `{}`",
            config.get_concealed_connection_string()?
        );

        let mut builder = Config {
            url: Some(config.get_connection_string()?),
            ..Default::default()
        }
        .builder(Some(Runtime::Tokio1));

        if let Some(max) = config.pool.max_connections {
            builder = builder.max_size(max);
        }

        if let Some(timeout) = config.pool.max_lifetime_ms {
            builder =
                builder.recycle_timeout(Some(Duration::from_millis(timeout)));
        }

        let pool = builder.build()?;

        Ok(Self {
            config,
            pool,
            topics: Default::default(),
        })
    }

    pub(super) async fn publish_raw(
        &self,
        topic: &str,
        key: Option<&str>,
        body: &[u8],
    ) -> AppResult<()> {
        let (_, chan) = self.acquire_channel().await?;
        let key = key.unwrap_or("#");

        let topic = self.ensure_topic_created(&chan, topic).await?;

        if self.config.require_ack {
            topic.publish_raw(&chan, key, body).await?;
        } else {
            topic.publish_raw_confirmed(&chan, key, body).await?;
        }

        Ok(())
    }

    pub(super) async fn publish<T: Serialize>(
        &self,
        topic: &str,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()> {
        let (_, chan) = self.acquire_channel().await?;
        let key = key.unwrap_or("#");

        let topic = self.ensure_topic_created(&chan, topic).await?;

        if self.config.require_ack {
            topic.publish(&chan, key, body).await?;
        } else {
            topic.publish_confirmed(&chan, key, body).await?;
        }

        Ok(())
    }

    pub(super) fn subscribe<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> BoxStream<'a, AppResult<T>> {
        stream! {
            let (_, chan) = self.acquire_channel().await?;
            let key = key.unwrap_or("#");

            let topic = self.ensure_topic_created(&chan, topic).await?;
            let mut stream = topic.subscribe::<T>(&chan, key);

            while let Some(i) = stream.next().await {
                yield i;
            }
        }
        .boxed()
    }

    async fn ensure_topic_created(
        &self,
        chan: &Channel,
        name: &str,
    ) -> AppResult<Arc<RabbitMQTopic>> {
        if let Some(topic) = self.topics.read().await.get(name) {
            return Ok(topic.clone());
        };

        let topic =
            Arc::new(RabbitMQTopic::create(name.to_owned(), &chan).await?);

        self.topics
            .write()
            .await
            .insert(name.to_owned(), topic.clone());

        Ok(topic)
    }

    async fn acquire_channel(&self) -> AppResult<(Object, Channel)> {
        let conn = self.pool.get().await.map_err(map_ipc_error)?;
        let chan = conn.create_channel().await.map_err(map_ipc_error)?;

        Ok((conn, chan))
    }
}
