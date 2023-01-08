mod channels_listener;

use std::sync::Arc;

use chrono::Utc;
use kernel_entities::{
    entities::comm::{Chat, Message},
    traits::Key,
};
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
    listener: ChatsChannelsListener<IPC>,
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
        let update = OutgoingChannelUpdate {
            user_id: todo!(),
            channel_id: todo!(),
            kind: OutgoingChannelUpdateKind::Message {
                platform_chat_id: "test_chat_id,".to_owned(),
                platform_user_id: "test_user_id".to_owned(),
                kind: OutgoingMessageUpdateKind::New {
                    content: text.to_owned(),
                },
                timestamp: Utc::now(),
            },
        };

        self.listener.enqueue_update(update)?;

        todo!()
    }
}

impl<IPC: MessagePassingService> AppChatsService<IPC> {
    pub async fn create(
        ipc: Arc<IPC>,
        channels_svc: Arc<dyn ChannelsService>,
    ) -> AppResult<Self> {
        let listener = ChatsChannelsListener::create(ipc, channels_svc).await?;

        Ok(Self { listener })
    }
}

#[async_trait::async_trait]
impl<IPC: MessagePassingService> Service for AppChatsService<IPC> {
    async fn initialize(&self) -> AppResult<()> {
        Ok(())
    }
}
