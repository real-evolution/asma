use derive_more::Constructor;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, ChatState},
        link::{Channel, Instance},
    },
    traits::Key,
};

use crate::traits::*;

#[async_trait::async_trait]
pub trait ChatsRepo: Repo<Entity = Chat> + Send + Sync {}

#[derive(Constructor)]
pub struct InsertChat {
    pub user_id: Key<User>,
    pub label: Option<String>,
    pub state: ChatState,
    pub channel_id: Key<Channel>,
    pub instance_id: Key<Instance>,
}
