use futures::stream::BoxStream;
use kernel_services::{
    error::{AppError, AppResult},
    link::error::LinkError,
};
use teloxide::{
    requests::Requester,
    types::{MediaKind, MessageKind, Update, UpdateKind},
    Bot,
};

use super::util::map_request_error;
use crate::link::channels::handlers::updates::*;

impl IncomingHandlerUpdate {
    pub(super) fn read_from<'a>(
        bot: &'a Bot,
    ) -> BoxStream<'a, AppResult<Self>> {
        Box::pin(async_stream::stream! {
        let mut i: i32 = 0;

        loop {
            let mut req = bot.get_updates();

            req.offset = Some(i);

            for update in req.await.map_err(map_request_error)? {
                i = update.id + 1;

                yield update.try_into();
            }
        }
        })
    }

    fn from_telegram_message<
        M: FnOnce(String, String) -> IncomingMessageUpdateKind,
    >(
        value: teloxide::prelude::Message,
        mapper: M,
    ) -> AppResult<Self> {
        match value.kind {
            | MessageKind::Common(inner) => {
                let Some(from) = inner.from else {
                    return Err(LinkError::UnsupportedEvent("only private chats are supported".into()).into()); 
                };

                let MediaKind::Text(content) = inner.media_kind else {
                    return Err(LinkError::UnsupportedEvent("only text messages supported".into()).into()); 
                };

                let chat_id = value.chat.id.0.to_string();
                let message_id = value.id.0.to_string();
                let by_id = from.id.0.to_string();

                Ok(Self::Message(IncomingMessageUpdate {
                    chat_id,
                    by_id,
                    kind: mapper(message_id, content.text),
                    sent_at: value.date,
                }))
            }

            | _ => Err(LinkError::UnsupportedEvent(format!(
                "unsupported telegram update: {:#?}",
                value
            ))
            .into()),
        }
    }
}

impl TryFrom<Update> for IncomingHandlerUpdate {
    type Error = AppError;

    fn try_from(value: Update) -> Result<Self, Self::Error> {
        match value.kind {
            | UpdateKind::Message(msg) => {
                Self::from_telegram_message(msg, |message_id, text| {
                    IncomingMessageUpdateKind::New {
                        message_id,
                        content: Some(text),
                    }
                })
            }
            | UpdateKind::EditedMessage(edit) => {
                Self::from_telegram_message(edit, |message_id, text| {
                    IncomingMessageUpdateKind::Edit {
                        message_id,
                        content: Some(text),
                    }
                })
            }
            | UpdateKind::Error(err) => {
                Err(LinkError::Communication(err.to_string()).into())
            }
            | _ => Err(LinkError::UnsupportedEvent(format!(
                "unsupported telegram update: {:#?}",
                value
            ))
            .into()),
        }
    }
}
