use kernel_entities::entities::auth::*;
use kernel_entities::entities::link::Channel;
use kernel_entities::traits::Key;

use crate::error::AppResult;
use crate::link::models::ChannelInfo;

#[async_trait::async_trait]
pub trait ChannelsService: Send + Sync {
    async fn create_telegram_channel_for(
        &self,
        user_id: &Key<User>,
        info: ChannelInfo,
    ) -> AppResult<Channel>;

    async fn create_whatsapp_channel_for(
        &self,
        user_id: &Key<User>,
        info: ChannelInfo,
    ) -> AppResult<Channel>;

    async fn toggle_channel(
        &self,
        channel_id: &Key<Channel>,
        is_active: bool,
    ) -> AppResult<()>;

    async fn start_channels(&self);
    async fn stop_channels(&self);
}
