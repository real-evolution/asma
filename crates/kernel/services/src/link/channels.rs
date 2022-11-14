use kernel_entities::entities::{auth::UserKey, link::ChannelKey};
use shaku::Interface;

use crate::error::AppResult;

use crate::link::models::ChannelInfo;

#[async_trait::async_trait]
pub trait ChannelsService: Interface {
    async fn create_telegram_channel_for(
        &self,
        user_id: &UserKey,
        info: ChannelInfo,
    ) -> AppResult<ChannelKey>;

    async fn create_whatsapp_channel_for(
        &self,
        user_id: &UserKey,
        info: ChannelInfo,
    ) -> AppResult<ChannelKey>;

    async fn toggle_channel(
        &self,
        channel_id: &ChannelKey,
        is_active: bool,
    ) -> AppResult<()>;

    async fn start_channels(&self);
    async fn stop_channels(&self);
}
