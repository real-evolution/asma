use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    time::Duration,
};

use common_async_utils::queue::BoundedQueue;
use kernel_entities::entities::link::Channel;
use kernel_services::{
    error::AppResult,
    link::{
        channels::{
            IncomingChannelUpdateKind,
            IncomingMessageUpdateKind,
            OutgoingChannelUpdateKind,
            OutgoingMessageUpdateKind,
        },
        error::LinkError,
    },
};
use teloxide::{
    requests::Requester,
    types::{MediaKind, Message, MessageKind, Update, UpdateKind, UserId},
    Bot,
};

use super::util::map_request_error;
use crate::link::channels::channel_stream::ChannelStream;

pub(crate) struct TelegramStream {
    bot: Bot,
    update_idx: AtomicI32,
    in_buf: BoundedQueue<IncomingChannelUpdateKind>,
}

#[async_trait::async_trait]
impl ChannelStream for TelegramStream {
    async fn recv(&self) -> AppResult<IncomingChannelUpdateKind> {
        self.read_next_update().await
    }

    async fn send(&self, update: OutgoingChannelUpdateKind) -> AppResult<()> {
        self.send_update(update).await
    }
}

impl TelegramStream {
    pub(crate) fn new(channel: &Channel) -> AppResult<Arc<Self>> {
        let client = teloxide::net::default_reqwest_settings()
            .timeout(Duration::from_secs(180))
            .connect_timeout(Duration::from_secs(300))
            .build()
            .map_err(|err| LinkError::Communication(err.to_string()))?;

        let bot = Bot::with_client(&channel.api_key, client);

        Ok(Arc::new(Self {
            bot,
            update_idx: 0.into(),
            in_buf: BoundedQueue::new(1024),
        }))
    }

    #[inline]
    async fn read_next_update(&self) -> AppResult<IncomingChannelUpdateKind> {
        loop {
            if let Ok(update) = self.in_buf.try_dequeue().await {
                return Ok(update);
            };

            let mut req = self.bot.get_updates();

            req.offset = Some(self.update_idx.load(Ordering::Acquire));

            for update in req.await.map_err(map_request_error)? {
                self.update_idx.store(update.id + 1, Ordering::Relaxed);

                let item = self.convert_from_telegram_update(update)?;

                if let Err(err) = self.in_buf.enqueue(item).await {
                    warn!("error enqueuing item: {err:#?}");
                }
            }
        }
    }

    #[inline]
    async fn send_update(
        &self,
        update: OutgoingChannelUpdateKind,
    ) -> AppResult<()> {
        match update {
            | OutgoingChannelUpdateKind::Message {
                platform_user_id,
                kind,
                timestamp: _,
            } => match kind {
                | OutgoingMessageUpdateKind::New { content } => {
                    self.bot
                        .send_message(UserId(platform_user_id as u64), content)
                        .await
                        .map_err(map_request_error)?;

                    Ok(())
                }
            },
        }
    }

    fn convert_from_telegram_update(
        &self,
        update: Update,
    ) -> AppResult<IncomingChannelUpdateKind> {
        match update.kind {
            | UpdateKind::Message(msg) => {
                self.convert_from_telegram_message(msg)
            }
            | UpdateKind::Error(err) => {
                Err(LinkError::Communication(err.to_string()).into())
            }
            | _ => Err(LinkError::UnsupportedEvent(format!(
                "unsupported telegram update: {update:#?}"
            ))
            .into()),
        }
    }

    fn convert_from_telegram_message(
        &self,
        message: Message,
    ) -> AppResult<IncomingChannelUpdateKind> {
        let MessageKind::Common(inner) = message.kind else {
            return Err(LinkError::UnsupportedEvent(format!("unsupported telegram update: {:?}", message.kind)).into());
        };

        let Some(from) = inner.from else {
            return Err(LinkError::UnsupportedEvent("only private chats are supported".into()).into());
        };

        let MediaKind::Text(content) = inner.media_kind else {
            return Err(LinkError::UnsupportedEvent("only text messages supported".into()).into());             
        };

        let platform_user_id = from.id.0 as i64;
        let timestamp = message.date;
        let kind = IncomingMessageUpdateKind::New {
            content: Some(content.text),
        };

        Ok(IncomingChannelUpdateKind::Message {
            platform_user_id,
            kind,
            timestamp,
        })
    }
}
