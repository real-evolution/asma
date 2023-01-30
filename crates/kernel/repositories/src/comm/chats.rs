use derive_more::Constructor;
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, ChatState, Message},
    },
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait ChatsRepo:
    Repo<Entity = Chat> + InsertRepo<InsertChat> + ChildRepo<User> + Send + Sync
{
    async fn watch(
        &self,
        id: &Key<Chat>,
    ) -> RepoResult<BoxStream<'_, RepoResult<Message>>>;

    async fn watch_all_of(
        &self,
        user_id: &Key<User>,
    ) -> RepoResult<BoxStream<'static, RepoResult<Message>>>;
}

#[derive(Constructor)]
pub struct InsertChat {
    pub label: Option<String>,
    pub state: ChatState,
    pub user_id: Key<User>,
}
