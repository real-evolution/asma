use std::collections::HashMap;

use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};

use super::models::ChannelStatus;
use crate::error::AppResult;

#[async_trait::async_trait]
pub trait ChannelsService: Send + Sync {
    async fn status(
        &self,
        id: &Key<Channel>,
    ) -> AppResult<Option<ChannelStatus>>;

    async fn status_of(
        &self,
        user_id: &Key<User>,
    ) -> AppResult<HashMap<Key<Channel>, ChannelStatus>>;

    async fn start_channels(&self);
    async fn stop_channels(&self);
}
