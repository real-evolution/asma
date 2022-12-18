use chrono::{DateTime, Utc};
use derive_more::Constructor;
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{auth::User, link::*},
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait ChannelsRepo:
    Repo<Entity = Channel>
    + InsertRepo<InsertChannel>
    + ChildRepo<User>
    + Send
    + Sync
{
    async fn stream_active<'a>(&'a self) -> BoxStream<'a, RepoResult<Channel>>;

    async fn stream_active_of<'a>(
        &'a self,
        user_id: Key<User>,
    ) -> BoxStream<'a, RepoResult<Channel>>;
}

#[derive(Constructor)]
pub struct InsertChannel {
    pub user_id: Key<User>,
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
}
