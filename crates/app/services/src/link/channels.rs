use std::sync::Arc;

use chrono::Utc;
use kernel_entities::entities::link::ChannelPlatform;
use kernel_entities::entities::{auth::UserKey, link::ChannelKey};
use kernel_repositories::link::{ChannelsRepo, InsertChannel};
use kernel_services::error::AppResult;
use kernel_services::link::{channels::ChannelsService, models::ChannelInfo};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = ChannelsService)]
pub struct AppChannelsService {
    #[shaku(inject)]
    channels: Arc<dyn ChannelsRepo>,
}

#[async_trait::async_trait()]
impl ChannelsService for AppChannelsService {
    async fn create_telegram_channel_for(
        &self,
        user_id: &UserKey,
        info: ChannelInfo,
    ) -> AppResult<ChannelKey> {
        Ok(self
            .channels
            .create(InsertChannel::new(
                info.name,
                ChannelPlatform::Telegram,
                info.api_key,
                Some(Utc::now() + info.valid_for),
                true,
                user_id.clone(),
            ))
            .await?)
    }

    async fn create_whatsapp_channel_for(
        &self,
        user_id: &UserKey,
        info: ChannelInfo,
    ) -> AppResult<ChannelKey> {
        Ok(self
            .channels
            .create(InsertChannel::new(
                info.name,
                ChannelPlatform::WhatsApp,
                info.api_key,
                Some(Utc::now() + info.valid_for),
                true,
                user_id.clone(),
            ))
            .await?)
    }

    async fn toggle_channel(
        &self,
        channel_id: &ChannelKey,
        is_active: bool,
    ) -> AppResult<()> {
        todo!()
    }

    async fn start_channels(&self) {
        todo!()
    }

    async fn stop_channels(&self) {
        todo!()
    }
}
