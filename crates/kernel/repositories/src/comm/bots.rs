use derive_more::Constructor;
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{auth::User, comm::Bot},
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait BotsRepo:
    Repo<Entity = Bot> + InsertRepo<InsertBot> + ChildRepo<User> + Send + Sync
{
    fn stream_active(&self) -> BoxStream<'_, RepoResult<Bot>>;
}

#[derive(Constructor)]
pub struct InsertBot {
    pub name: String,
    pub is_active: bool,
    pub user_id: Key<User>,
}
