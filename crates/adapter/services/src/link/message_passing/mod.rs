mod config;
mod topic;
mod util;

use std::{collections::HashMap, sync::Arc, time::Duration};

use deadpool_lapin::{Config, Pool, Runtime};
use kernel_services::{
    config::ConfigService,
    error::AppResult,
    link::message_passing::MessagePassingService,
    Service,
};
use tokio::sync::RwLock;
pub use topic::RabbitMqTopic;

use self::config::{MessageQueueConfig, MESSAGE_QUEUE_CONFIG_SECTION};
use crate::link::message_passing::util::map_ipc_error;

pub struct RabbitMqMessagePassingService {
    pool: Pool,
    topics: RwLock<HashMap<String, Arc<RabbitMqTopic>>>,
}

#[async_trait::async_trait]
impl MessagePassingService for RabbitMqMessagePassingService {
    type TopicType = RabbitMqTopic;

    async fn get_topic(&self, name: &str) -> AppResult<Arc<RabbitMqTopic>> {
        if let Some(topic) = self.topics.read().await.get(name) {
            return Ok(topic.clone());
        };

        let topic = Arc::new(
            RabbitMqTopic::create(name.to_owned(), self.pool.clone()).await?,
        );

        self.topics
            .write()
            .await
            .insert(name.to_owned(), topic.clone());

        Ok(topic)
    }
}

impl RabbitMqMessagePassingService {
    pub async fn create<C: ConfigService>(config: Arc<C>) -> AppResult<Self> {
        let conf: MessageQueueConfig =
            config.get_section(MESSAGE_QUEUE_CONFIG_SECTION)?;

        debug!(
            "openning rabbitmq connection to: `{}`",
            conf.get_concealed_connection_string()?
        );

        let mut builder = Config {
            url: Some(conf.get_connection_string()?),
            ..Default::default()
        }
        .builder(Some(Runtime::Tokio1));

        if let Some(max) = conf.pool.max_connections {
            builder = builder.max_size(max);
        }

        if let Some(timeout) = conf.pool.max_lifetime_ms {
            builder =
                builder.recycle_timeout(Some(Duration::from_millis(timeout)));
        }

        let pool = builder.build().map_err(map_ipc_error)?;

        Ok(Self {
            pool,
            topics: Default::default(),
        })
    }
}

#[async_trait::async_trait]
impl Service for RabbitMqMessagePassingService {
    async fn initialize(&self) -> AppResult<()> {
        Ok(())
    }
}
