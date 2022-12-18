mod incoming;
mod outgoing;
mod util;

use futures::stream::BoxStream;
use kernel_entities::entities::link::Channel;
use kernel_services::error::AppResult;
use teloxide::Bot;

use super::{updates::*, ChannelHandler};

pub(super) struct TelegramHandler {
    bot: Bot,
}

#[async_trait::async_trait]
impl ChannelHandler for TelegramHandler {
    async fn updates<'a>(
        &'a self,
    ) -> BoxStream<'a, AppResult<IncomingHandlerUpdate>> {
        IncomingHandlerUpdate::read_from(&self.bot)
    }

    async fn send(&self, update: OutgoingHandlerUpdate) -> AppResult<()> {
        Ok(update.perform(&self.bot).await?)
    }
}

impl TelegramHandler {
    pub(super) fn new(channel: &Channel) -> Self {
        let bot = Bot::new(&channel.api_key);

        Self { bot }
    }
}
