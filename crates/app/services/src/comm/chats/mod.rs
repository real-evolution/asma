mod channels_listener;

use std::sync::Arc;

use chrono::Utc;
use kernel_entities::{
    entities::comm::{Chat, Message},
    traits::Key,
};
use kernel_repositories::DataStore;
use kernel_services::{
    comm::chats::ChatsService,
    error::AppResult,
    link::{
        channels::{
            ChannelPipe, ChannelsService, OutgoingChannelUpdate,
            OutgoingChannelUpdateKind, OutgoingMessageUpdateKind,
        },
        message_passing::MessagePassingService,
    },
    Service,
};

use self::channels_listener::ChatsChannelsListener;

pub struct AppChatsService<IPC> {
}
#[async_trait::async_trait]
impl<IPC> ChatsService for AppChatsService<IPC>
where
    IPC: MessagePassingService,
{
    async fn send_message(
        &self,
        chat_id: &Key<Chat>,
        text: &str,
    ) -> AppResult<Message> {
        todo!()
    }
}
#[async_trait::async_trait]
}
