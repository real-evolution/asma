use derive_more::Constructor;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, ChatState},
    },
    traits::Key,
};

use crate::traits::*;

pub trait ChatsRepo:
    Repo<Entity = Chat> + InsertRepo<InsertChat> + Send + Sync
{
}

#[derive(Constructor)]
pub struct InsertChat {
    pub label: Option<String>,
    pub state: ChatState,
    pub user_id: Key<User>,
}
