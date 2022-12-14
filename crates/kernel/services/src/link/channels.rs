use futures::stream::BoxStream;
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

    async fn status_of<'a>(
        &'a self,
        user_id: &'a Key<User>,
    ) -> AppResult<BoxStream<'a, (Key<Channel>, ChannelStatus)>>;

    async fn start_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>>;
    async fn stop_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>>;
}
