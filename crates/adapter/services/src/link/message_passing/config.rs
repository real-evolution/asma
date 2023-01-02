use anyhow::Result;
use common_macros::into_fn;
use common_validation::*;
use serde::Deserialize;
use validator::Validate;

pub(super) const MESSAGE_QUEUE_CONFIG_SECTION: &str = "message_queue";

into_fn!(default_require_ack: const bool => true);

#[derive(Debug, Deserialize, Validate)]
pub(super) struct MessageQueueConfig {
    #[validate(custom = "supported_message_queue_protocol")]
    pub protocol: String,
    #[validate(custom = "endpoint")]
    pub host: String,
    #[validate(range(min = 0, max = 0xFFFF))]
    pub port: Option<u16>,
    #[validate(custom = "identifier")]
    pub username: String,
    pub password: String,
    #[validate]
    pub pool: PoolConfig,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub(super) struct PoolConfig {
    #[validate(range(min = 1, max = 64))]
    pub max_connections: Option<usize>,
    pub max_lifetime_ms: Option<u64>,
}

impl MessageQueueConfig {
    pub(super) fn get_connection_string(&self) -> Result<String> {
        self.do_get_connection_string::<false>()
    }

    pub(super) fn get_concealed_connection_string(&self) -> Result<String> {
        self.do_get_connection_string::<true>()
    }

    fn do_get_connection_string<const CONCEALED: bool>(
        &self,
    ) -> Result<String> {
        let ep = Endpoint::parse_str(&self.host)?;
        let host = match self.port.or(ep.port) {
            | Some(port) => format!("{}:{}", ep.domain, port),
            | None => ep.domain,
        };

        let password = if CONCEALED { "***" } else { &self.password };

        Ok(format!(
            "{}://{}:{}@{}/%2f",
            self.protocol, self.username, password, host
        ))
    }
}
