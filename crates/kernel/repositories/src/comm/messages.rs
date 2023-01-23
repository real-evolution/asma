use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, Message, MessageDirection},
        link::Instance,
    },
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait MessagesRepo:
    Repo<Entity = Message>
    + InsertRepo<InsertMessage>
    + ChildRepo<Chat>
    + Send
    + Sync
{
    async fn update_text(
        &self,
        id: &Key<Message>,
        new_text: Option<String>,
    ) -> RepoResult<Message>;
}

#[derive(Clone, Debug, Constructor)]
pub struct InsertMessage {
    pub text: Option<String>,
    pub direction: MessageDirection,
    pub delivered_at: DateTime<Utc>,
    pub user_id: Key<User>,
    pub chat_id: Key<Chat>,
    pub instance_id: Key<Instance>,
}
