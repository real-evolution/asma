mod channels_listener;

use std::sync::Arc;

use chrono::Utc;
use kernel_entities::{entities::comm::Chat, traits::Key};
use kernel_repositories::{DataStore, DocumentStore};
use kernel_services::{
    comm::chats::ChatsService,
    error::AppResult,
    link::{
        channels::{
            ChannelsService, OutgoingChannelUpdate, OutgoingChannelUpdateKind,
            OutgoingMessageUpdateKind,
        },
        message_passing::MessagePassingService,
    },
    Service,
};

use self::channels_listener::ChatsChannelsListener;

pub struct AppChatsService<IPC> {
    data: Arc<dyn DataStore>,
    docs: Arc<dyn DocumentStore>,
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
    ) -> AppResult<()> {
        let chat = self.docs.chats().get(chat_id).await?;
        let instance =
            self.data.link().instances().get(&chat.instance_id).await?;

        let update = OutgoingChannelUpdate {
            user_id: chat.user_id,
            channel_id: chat.channel_id,
            kind: OutgoingChannelUpdateKind::Message {
                platform_user_id: instance.platform_identifier,
                kind: OutgoingMessageUpdateKind::New {
                    content: text.to_owned(),
                },
                timestamp: Utc::now(),
            },
        };

        self.listener.enqueue_update(update)?;

        Ok(())
    }
}

impl<IPC: MessagePassingService> AppChatsService<IPC> {
    pub async fn create(
        ipc: Arc<IPC>,
        data: Arc<dyn DataStore>,
        docs: Arc<dyn DocumentStore>,
        channels_svc: Arc<dyn ChannelsService>,
    ) -> AppResult<Self> {
        let listener =
            ChatsChannelsListener::create(ipc, docs.clone(), channels_svc)
                .await?;

        Ok(Self {
            data,
            docs,
            listener,
        })
    }
}

#[async_trait::async_trait]
impl<IPC: MessagePassingService> Service for AppChatsService<IPC> {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        Ok(())
    }
}
