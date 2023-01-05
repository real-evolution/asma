use chrono::{DateTime, Utc};
use derive_more::Constructor;
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{auth::User, link::*},
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

pub trait ChannelsRepo:
    Repo<Entity = Channel>
    + InsertRepo<InsertChannel>
    + ChildRepo<User>
    + Send
    + Sync
{
    fn stream_active(&self) -> BoxStream<'_, RepoResult<Channel>>;
    fn stream_active_of(
        &self,
        user_id: Key<User>,
    ) -> BoxStream<'_, RepoResult<Channel>>;
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
