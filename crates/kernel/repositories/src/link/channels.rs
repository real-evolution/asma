use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{
    entities::{auth::User, link::*},
    traits::Key,
};
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait ChannelsRepo: Interface {
    async fn get_by_id(&self, id: &Key<Channel>) -> RepoResult<Channel>;
    async fn create(&self, insert: InsertChannel) -> RepoResult<Key<Channel>>;
}

#[derive(Constructor)]
pub struct InsertChannel {
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub user_id: Key<User>,
}
