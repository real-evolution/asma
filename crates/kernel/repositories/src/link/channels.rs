use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::entities::{auth::UserKey, link::*};
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait ChannelsRepo: Interface {
    async fn get_by_id(&self, id: &ChannelKey) -> RepoResult<Channel>;
    async fn create(&self, insert: InsertChannel) -> RepoResult<ChannelKey>;
}

#[derive(Constructor, Debug)]
pub struct InsertChannel {
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
    pub valid_until: Option<DateTime<Utc>>,
    pub user_id: UserKey,
}
