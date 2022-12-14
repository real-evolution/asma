use kernel_services::{error::AppResult, link::error::LinkError};
use teloxide::{requests::Requester, types::MessageId, Bot};

use super::util::map_request_error;
use crate::link::channels::handlers::updates::*;

impl OutgoingHandlerUpdate {
    pub(super) async fn perform(self, bot: &Bot) -> AppResult<()> {
        match self {
            | OutgoingHandlerUpdate::Message(msg) => match msg.kind {
                | OutgoingMessageUpdateKind::New { content } => {
                    bot.send_message(msg.chat_id, content)
                        .await
                        .map_err(map_request_error)?;

                    Ok(())
                }
                | OutgoingMessageUpdateKind::Edit {
                    message_id,
                    content,
                } => {
                    let Ok(message_id) = i32::from_str_radix(&message_id, 10) else {
                        return Err(LinkError::InvalidParams(format!("invalid message id: {message_id}")).into());
                    };

                    bot.edit_message_text(
                        msg.chat_id,
                        MessageId(message_id),
                        content.unwrap_or(String::new()),
                    )
                    .await
                    .map_err(map_request_error)?;

                    Ok(())
                }
                | OutgoingMessageUpdateKind::Delete { message_id } => {
                    let Ok(message_id) = i32::from_str_radix(&message_id, 10) else {
                        return Err(LinkError::InvalidParams(format!("invalid message id: {message_id}")).into());
                    };

                    bot.delete_message(msg.chat_id, MessageId(message_id))
                        .await
                        .map_err(map_request_error)?;

                    Ok(())
                }
            },
        }
    }
}
