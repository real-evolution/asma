use std::{
    collections::HashSet,
    marker::PhantomData,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use deadpool_lapin::{Object, Pool};
use derive_more::Constructor;
use futures::{stream::BoxStream, StreamExt};
use kernel_services::{
    error::AppResult,
    link::message_passing::{
        MessageConfirmation, Topic, TopicReader, TopicWriter,
    },
};
use lapin::{
    acker::Acker,
    message::Delivery,
    options::{
        BasicConsumeOptions, BasicNackOptions, ExchangeDeclareOptions,
        QueueDeclareOptions,
    },
    publisher_confirm::PublisherConfirm,
    Channel, ExchangeKind,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;

use super::util::{deserialize, map_ipc_error, map_params_error};

#[derive(Debug)]
pub(super) struct RabbitMqTopic {
    name: String,
    pool: Pool,
    queues: RwLock<HashSet<String>>,
    current_consumer_id: AtomicU32,
}

#[derive(Debug)]
pub(super) struct RabbitMqTopicWrapper<T> {
    inner: Arc<RabbitMqTopic>,
    _phantom: PhantomData<T>,
}

impl RabbitMqTopic {
    pub(super) async fn create(name: String, pool: Pool) -> AppResult<Self> {
        let (_, ch) = Self::acquire_channel(&pool).await?;

        ch.exchange_declare(
            &name,
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                ..Default::default()
            },
            Default::default(),
        )
        .await
        .map_err(map_ipc_error)?;

        Ok(Self {
            name,
            pool,
            queues: Default::default(),
            current_consumer_id: 0.into(),
        })
    }

    async fn do_publish<T: Serialize + Send + Sync>(
        &self,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<PublisherConfirm> {
        let buf = rmp_serde::to_vec(body).map_err(map_params_error)?;
        let key = format!("{}.{}", &self.name, key.unwrap_or("#"));

        let (_, ch) = Self::acquire_channel(&self.pool).await?;
        self.ensure_queue_created(&key, false, &ch).await?;

        let confirm = ch
            .basic_publish(
                &self.name,
                &key,
                Default::default(),
                &buf,
                Default::default(),
            )
            .await
            .map_err(map_ipc_error)?;

        Ok(confirm)
    }

    async fn do_subscribe<T, F: Fn(Delivery) -> AppResult<T>>(
        &self,
        key: Option<&str>,
        mirror: bool,
        manual_ack: bool,
        mapper: F,
    ) -> AppResult<impl tokio_stream::Stream<Item = AppResult<T>>> {
        let id = self
            .current_consumer_id
            .fetch_add(1, Ordering::AcqRel)
            .to_string();
        let key = format!("{}.{}", &self.name, key.unwrap_or("#"));

        let opts = BasicConsumeOptions {
            exclusive: mirror,
            no_ack: !manual_ack,
            ..Default::default()
        };

        let (_, ch) = Self::acquire_channel(&self.pool).await?;
        let queue = self.ensure_queue_created(&key, mirror, &ch).await?;

        Ok(ch
            .basic_consume(&queue, &id, opts, Default::default())
            .await
            .map_err(map_ipc_error)?
            .map(move |i| match i {
                | Ok(i) => mapper(i),
                | Err(err) => Err(map_ipc_error(err)),
            }))
    }

    async fn ensure_queue_created<'a>(
        &'a self,
        key: &str,
        temporary: bool,
        ch: &Channel,
    ) -> AppResult<String> {
        let mut queues = self.queues.write().await;

        if !queues.contains(key) {
            queues.insert(create_queue(&self.name, key, true, ch).await?);
        }

        async fn create_queue(
            topic_name: &str,
            key: &str,
            temporary: bool,
            ch: &Channel,
        ) -> AppResult<String> {
            let declare_opts = QueueDeclareOptions {
                durable: !temporary,
                auto_delete: !temporary,
                ..Default::default()
            };

            let queue = ch
                .queue_declare(
                    if temporary { "" } else { key },
                    declare_opts,
                    Default::default(),
                )
                .await
                .map_err(map_ipc_error)?;

            ch.queue_bind(
                queue.name().as_str(),
                topic_name,
                key,
                Default::default(),
                Default::default(),
            )
            .await
            .map_err(map_ipc_error)?;

            Ok(queue.name().to_string())
        }

        if temporary {
            create_queue(&self.name, key, true, ch).await
        } else {
            Ok(key.to_string())
        }
    }

    async fn acquire_channel(pool: &Pool) -> AppResult<(Object, Channel)> {
        let conn = pool.get().await.map_err(map_ipc_error)?;
        let chan = conn.create_channel().await.map_err(map_ipc_error)?;

        Ok((conn, chan))
    }
}
#[async_trait::async_trait]
impl<T> TopicWriter<T> for RabbitMqTopicWrapper<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    async fn publish(&self, key: Option<&str>, body: &T) -> AppResult<()> {
        self.inner.do_publish(key, body).await?;

        Ok(())
    }

    async fn publish_confirmed(
        &self,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()> {
        self.inner
            .do_publish(key, body)
            .await?
            .await
            .map_err(map_ipc_error)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T> TopicReader<T> for RabbitMqTopicWrapper<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    async fn subscribe(
        &self,
        key: Option<&str>,
    ) -> AppResult<BoxStream<'_, AppResult<T>>> {
        Ok(self
            .inner
            .do_subscribe(key, false, false, |i| deserialize::<T>(&i.data))
            .await?
            .boxed())
    }

    async fn subscribe_manual(
        &self,
        key: Option<&str>,
    ) -> AppResult<BoxStream<'_, AppResult<(T, Arc<dyn MessageConfirmation>)>>>
    {
        Ok(self
            .inner
            .do_subscribe(key, false, true, |i| {
                let confirm: Arc<dyn MessageConfirmation> =
                    Arc::new(RabbitMQMessageConfirmation::new(i.acker));

                Ok((deserialize::<T>(&i.data)?, confirm))
            })
            .await?
            .boxed())
    }

    async fn mirror(
        &self,
        key: Option<&str>,
    ) -> AppResult<BoxStream<'_, AppResult<T>>> {
        Ok(self
            .inner
            .do_subscribe(key, true, false, |i| deserialize::<T>(&i.data))
            .await?
            .boxed())
    }
}

impl<T> RabbitMqTopicWrapper<T>
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
{
    pub(super) fn new_arc(value: Arc<RabbitMqTopic>) -> Arc<dyn Topic<T>> {
        Arc::new(RabbitMqTopicWrapper {
            inner: value,
            _phantom: PhantomData,
        })
    }
}

#[derive(Constructor)]
pub(super) struct RabbitMQMessageConfirmation {
    acker: Acker,
}

#[async_trait::async_trait]
impl MessageConfirmation for RabbitMQMessageConfirmation {
    async fn ack(&self) -> AppResult<()> {
        self.acker
            .ack(Default::default())
            .await
            .map_err(map_ipc_error)
    }

    async fn nack(&self, requeue: bool) -> AppResult<()> {
        self.acker
            .nack(BasicNackOptions {
                multiple: false,
                requeue,
            })
            .await
            .map_err(map_ipc_error)
    }
}
