use std::{
    collections::HashSet,
    sync::atomic::{AtomicU32, Ordering},
};

use futures::StreamExt;
use kernel_services::error::AppResult;
use lapin::{
    message::Delivery,
    publisher_confirm::PublisherConfirm,
    Channel,
    ExchangeKind,
};
use serde::Serialize;
use tokio::sync::RwLock;

use super::util::{map_ipc_error, map_params_error};

pub(super) struct RabbitMQTopic {
    name: String,
    queues: RwLock<HashSet<String>>,
    current_consumer_id: AtomicU32,
}

impl RabbitMQTopic {
    pub(super) async fn create(name: String, ch: &Channel) -> AppResult<Self> {
        ch.exchange_declare(
            &name,
            ExchangeKind::Topic,
            Default::default(),
            Default::default(),
        )
        .await
        .map_err(map_ipc_error)?;

        Ok(Self {
            name,
            queues: Default::default(),
            current_consumer_id: 0.into(),
        })
    }

    pub(super) async fn publish_raw(
        &self,
        ch: &Channel,
        key: &str,
        body: &[u8],
    ) -> AppResult<PublisherConfirm> {
        let confirm = ch
            .basic_publish(
                &self.name,
                key,
                Default::default(),
                body,
                Default::default(),
            )
            .await
            .map_err(map_ipc_error)?;

        Ok(confirm)
    }

    pub(super) async fn publish_raw_confirmed(
        &self,
        ch: &Channel,
        key: &str,
        body: &[u8],
    ) -> AppResult<()> {
        self.publish_raw(ch, key, body)
            .await?
            .await
            .map_err(map_ipc_error)?;

        Ok(())
    }

    pub(super) async fn publish<B: Serialize>(
        &self,
        ch: &Channel,
        key: &str,
        body: &B,
    ) -> AppResult<PublisherConfirm> {
        let buf = rmp_serde::to_vec(body).map_err(map_params_error)?;

        Ok(self.publish_raw(ch, key.into(), &buf).await?)
    }

    pub(super) async fn publish_confirmed<B: Serialize>(
        &self,
        ch: &Channel,
        key: &str,
        body: &B,
    ) -> AppResult<()> {
        self.publish(ch, key, body)
            .await?
            .await
            .map_err(map_ipc_error)?;

        Ok(())
    }

    pub(super) fn subscribe<'a>(
        &'a self,
        ch: &'a Channel,
        key: &'a str,
    ) -> impl tokio_stream::Stream<Item = AppResult<Delivery>> + 'a {
        async_stream::stream! {
            let id = self
                .current_consumer_id
                .fetch_add(1, Ordering::AcqRel)
                .to_string();

            self.ensure_queue_created(key, ch).await?;

            let mut stream = ch
                .basic_consume(key, &id, Default::default(), Default::default())
                .await
                .map_err(map_ipc_error)?;

            while let Some(i) = stream.next().await {
                yield i.map_err(map_ipc_error)
            }
        }
    }

    async fn ensure_queue_created(
        &self,
        key: &str,
        ch: &Channel,
    ) -> AppResult<()> {
        if self.queues.read().await.contains(key) {
            return Ok(());
        }

        ch.queue_declare(&key, Default::default(), Default::default())
            .await
            .map_err(map_ipc_error)?;

        ch.queue_bind(
            key,
            &self.name,
            key,
            Default::default(),
            Default::default(),
        )
        .await
        .map_err(map_ipc_error)?;

        self.queues.write().await.insert(key.to_owned());

        Ok(())
    }
}
