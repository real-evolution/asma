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

    fn status_of<'a>(
        &'a self,
        user_id: &'a Key<User>,
    ) -> BoxStream<'a, (Key<Channel>, ChannelStatus)>;

    fn start_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>>;
    fn stop_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>>;

    fn start_channels_of<'a>(
        &'a self,
        user_id: Key<User>,
    ) -> BoxStream<'a, AppResult<()>>;

    fn stop_channels_of<'a>(
        &'a self,
        user_id: Key<User>,
    ) -> BoxStream<'a, AppResult<()>>;
}
