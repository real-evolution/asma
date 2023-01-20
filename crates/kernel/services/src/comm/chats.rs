use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, Message, MessageDirection}, link::Instance,
    },
    traits::Key,
};

use crate::error::AppResult;

#[async_trait::async_trait]
pub trait ChatsService: Send + Sync {
    async fn send_message(
        &self,
        chat_id: &Key<Chat>,
        text: String,
    ) -> AppResult<()>;

    async fn watch_user_chats(
        &self,
        user_id: &Key<User>,
    ) -> AppResult<BoxStream<'static, AppResult<ChatEvent>>>;
}

#[derive(Debug)]
pub enum ChatEventKind {
    MessageAdded {
        id: Key<Message>,
        text: Option<String>,
        instance_id: Key<Instance>,
        direction: MessageDirection,
        created_at: DateTime<Utc>,
    },
}

#[derive(Debug)]
pub struct ChatEvent {
    pub chat_id: Key<Chat>,
    pub kind: ChatEventKind,
}
