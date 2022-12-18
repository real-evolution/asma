mod telegram;
mod updates;

use std::sync::Arc;

use futures::stream::BoxStream;
use kernel_entities::entities::link::{Channel, ChannelPlatform};
use kernel_services::error::AppResult;

#[async_trait::async_trait]
pub(crate) trait ChannelHandler: Send + Sync {
    async fn updates<'a>(
        &'a self,
    ) -> BoxStream<'a, AppResult<updates::IncomingHandlerUpdate>>;

    async fn send(
        &self,
        update: updates::OutgoingHandlerUpdate,
    ) -> AppResult<()>;
}

pub(super) fn create_handler(channel: &Channel) -> Arc<dyn ChannelHandler> {
    let handler = match channel.platform {
        | ChannelPlatform::Telegram => telegram::TelegramHandler::new(&channel),
    };

    Arc::new(handler)
}
