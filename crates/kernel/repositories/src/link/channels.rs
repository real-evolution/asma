use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{
    entities::{auth::User, link::*},
    traits::Key,
};
use shaku::Interface;

use crate::traits::repo::*;

#[async_trait::async_trait]
pub trait ChannelsRepo:
    Repo<Channel> + InsertRepo<Channel, InsertChannel> + Interface
{
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
