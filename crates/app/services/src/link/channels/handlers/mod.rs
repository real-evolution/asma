mod telegram;
mod updates;

use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use kernel_entities::entities::link::{Channel, ChannelPlatform};
use kernel_services::{error::AppResult, link::models::ChannelStatus};

#[async_trait::async_trait]
pub(crate) trait ChannelHandler: Send + Sync {
    async fn updates<'a>(
        &'a self,
    ) -> BoxStream<'a, AppResult<updates::IncomingHandlerUpdate>>;

    async fn send(
        &self,
        update: updates::OutgoingHandlerUpdate,
    ) -> AppResult<()>;

    async fn start(&self) -> AppResult<()>;
    async fn stop(&self) -> AppResult<()>;

    async fn running_since(&self) -> AppResult<Option<DateTime<Utc>>>;

    async fn status(&self) -> AppResult<Option<ChannelStatus>> {
        Ok(self
            .running_since()
            .await?
            .map(|ts| ChannelStatus { started_at: ts }))
    }
}

pub(super) fn create_handler(channel: Channel) -> Arc<dyn ChannelHandler> {
    let handler = match channel.platform {
        | ChannelPlatform::Telegram => telegram::TelegramHandler::new(channel),
    };

    Arc::new(handler)
}
