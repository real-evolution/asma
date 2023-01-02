use std::{collections::HashMap, sync::Arc, time::Duration};

use async_stream::stream;
use deadpool_lapin::{Config, Object, Pool, Runtime};
use derive_more::Constructor;
use futures::{stream::BoxStream, StreamExt};
use kernel_services::{
    error::AppResult,
    link::message_passing::MessageConfirmation,
};
use lapin::{
    acker::Acker,
    message::Delivery,
    options::BasicNackOptions,
    Channel,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{pin, sync::RwLock};

use super::{
    config::MessageQueueConfig,
    topic::RabbitMQTopic,
    util::{map_ipc_error, map_params_error},
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
        self.do_subscribe(topic, key)
            .map(|i| {
                i.map(|i| {
                    rmp_serde::from_slice(&i.data).map_err(map_params_error)
                })?
            })
            .boxed()
    }

    pub(super) fn subscribe_manual<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> BoxStream<'a, AppResult<(T, Arc<dyn MessageConfirmation>)>> {
        stream! {
            let stream = self.do_subscribe(topic, key);

            pin!(stream);

            while let Some(i) = stream.next().await {
                match i {
                    | Ok(i) => {
                        let body = rmp_serde::from_slice::<T>(&i.data)
                            .map_err(map_ipc_error)?;
                        let acker: Arc<dyn MessageConfirmation> =
                            Arc::new(RabbitMQMessageConfirmation::new(i.acker));

                        yield Ok((body, acker));
                    }
                    | Err(err) => yield Err(err)
                }
            }
        }
        .boxed::<'a>()
    }

    fn do_subscribe<'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> impl tokio_stream::Stream<Item = AppResult<Delivery>> + 'a {
        stream! {
            let (_, chan) = self.acquire_channel().await?;
            let key = key.unwrap_or("#");

            let topic = self.ensure_topic_created(&chan, topic).await?;
            let stream = topic.subscribe(&chan, key);

            pin!(stream);

            while let Some(i) = stream.next().await {
                yield i;
            }
        }
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

#[derive(Constructor)]
pub(super) struct RabbitMQMessageConfirmation {
    acker: Acker,
}

#[async_trait::async_trait]
impl MessageConfirmation for RabbitMQMessageConfirmation {
    async fn ack(self) -> AppResult<()> {
        self.acker
            .ack(Default::default())
            .await
            .map_err(map_ipc_error)
    }

    async fn nack(self, requeue: bool) -> AppResult<()> {
        self.acker
            .nack(BasicNackOptions {
                multiple: false,
                requeue,
            })
            .await
            .map_err(map_ipc_error)
    }
}
