use derive_more::Constructor;
use kernel_entities::{
    entities::comm::{Chat, Message, MessageDirection},
    traits::Key,
};

use crate::traits::*;

#[async_trait::async_trait]
pub trait MessagesRepo:
    Repo<Entity = Message> + InsertRepo<InsertMessage> + Send + Sync
{
}

#[derive(Clone, Debug, Constructor)]
pub struct InsertMessage {
    pub chat_id: Key<Chat>,
    pub text: Option<String>,
    pub direction: MessageDirection,
}
