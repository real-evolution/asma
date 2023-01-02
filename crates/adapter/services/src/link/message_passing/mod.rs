mod config;
mod context;
mod topic;
mod util;

use std::sync::Arc;

use futures::stream::BoxStream;
use kernel_services::{
    config::ConfigService,
    error::AppResult,
    link::message_passing::{MessageConfirmation, MessagePassingService},
    Service,
};
use serde::{de::DeserializeOwned, Serialize};

use self::{
    config::{MessageQueueConfig, MESSAGE_QUEUE_CONFIG_SECTION},
    context::RabbitMQContext,
};

pub struct RabbitMqMessagePassingService {
    ctx: RabbitMQContext,
}

#[async_trait::async_trait]
impl MessagePassingService for RabbitMqMessagePassingService {
    async fn publish_raw(
        &self,
        topic: &str,
        key: Option<&str>,
        body: &[u8],
    ) -> AppResult<()> {
        self.ctx.publish_raw(topic, key, body).await?;

        Ok(())
    }

    async fn publish<T: Serialize + Send + Sync>(
        &self,
        topic: &str,
        key: Option<&str>,
        body: &T,
    ) -> AppResult<()> {
        self.ctx.publish(topic, key, body).await?;

        Ok(())
    }

    fn subscribe<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> BoxStream<'a, AppResult<T>> {
        self.ctx.subscribe(topic, key)
    }

    fn subscribe_manual<'a, T: DeserializeOwned + Send + 'a>(
        &'a self,
        topic: &'a str,
        key: Option<&'a str>,
    ) -> BoxStream<'a, AppResult<(T, Arc<dyn MessageConfirmation>)>> {
        self.ctx.subscribe_manual(topic, key)
    }
}

impl RabbitMqMessagePassingService {
    pub async fn create<C: ConfigService>(conf: Arc<C>) -> AppResult<Self> {
        let conf: MessageQueueConfig =
            conf.get_section(MESSAGE_QUEUE_CONFIG_SECTION)?;

        let ctx = RabbitMQContext::create(conf).await?;

        Ok(Self { ctx })
    }
}

#[async_trait::async_trait]
impl Service for RabbitMqMessagePassingService {
    async fn initialize(&self) -> AppResult<()> {
        Ok(())
    }
}
