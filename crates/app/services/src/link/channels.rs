use std::sync::Arc;

use chrono::Utc;
use kernel_entities::entities::auth::User;
use kernel_entities::entities::link::{Channel, ChannelPlatform};
use kernel_entities::traits::Key;
use kernel_repositories::link::{ChannelsRepo, InsertChannel};
use kernel_services::error::AppResult;
use kernel_services::link::{channels::ChannelsService, models::ChannelInfo};

pub struct AppChannelsService {
    channels: Arc<dyn ChannelsRepo>,
}

#[async_trait::async_trait]
impl ChannelsService for AppChannelsService {
    async fn create_telegram_channel_for(
        &self,
        user_id: &Key<User>,
        info: ChannelInfo,
    ) -> AppResult<Channel> {
        Ok(self
            .channels
            .create(InsertChannel::new(
                user_id.clone(),
                info.name,
                ChannelPlatform::Telegram,
                info.api_key,
                Some(Utc::now() + info.valid_for),
                true,
            ))
            .await?)
    }

    async fn create_whatsapp_channel_for(
        &self,
        user_id: &Key<User>,
        info: ChannelInfo,
    ) -> AppResult<Channel> {
        Ok(self
            .channels
            .create(InsertChannel::new(
                user_id.clone(),
                info.name,
                ChannelPlatform::WhatsApp,
                info.api_key,
                Some(Utc::now() + info.valid_for),
                true,
            ))
            .await?)
    }

    async fn toggle_channel(
        &self,
        channel_id: &Key<Channel>,
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
