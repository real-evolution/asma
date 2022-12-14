mod incoming;
mod outgoing;
mod util;

use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use kernel_entities::entities::link::Channel;
use kernel_services::error::AppResult;
use teloxide::Bot;

use super::{updates::*, ChannelHandler};

pub(super) struct TelegramHandler {
    bot: Bot,
    channel: Channel,
    running_since: Option<DateTime<Utc>>,
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

    async fn start(&self) -> AppResult<()> {
        debug!(
            "starting channel `{}` ({})",
            self.channel.id, self.channel.name
        );

        todo!()
    }

    async fn stop(&self) -> AppResult<()> {
        debug!(
            "stopping channel `{}` ({})",
            self.channel.id, self.channel.name
        );

        todo!()
    }

    async fn running_since(&self) -> AppResult<Option<DateTime<Utc>>> {
        Ok(self.running_since)
    }
}

impl TelegramHandler {
    pub(super) fn new(channel: Channel) -> Self {
        let bot = Bot::new(&channel.api_key);

        Self {
            bot,
            channel,
            running_since: None,
        }
    }
}
