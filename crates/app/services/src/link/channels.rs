use std::{collections::HashMap, sync::Arc};

use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};
use kernel_repositories::DataStore;
use kernel_services::{
    error::AppResult,
    link::{channels::ChannelsService, models::ChannelStatus},
};

pub struct AppChannelsService {
    _data: Arc<dyn DataStore>,
    active_channels: HashMap<Key<User>, HashMap<Key<Channel>, ChannelStatus>>,
}

#[async_trait::async_trait]
impl ChannelsService for AppChannelsService {
    async fn status(
        &self,
        id: &Key<Channel>,
    ) -> AppResult<Option<ChannelStatus>> {
        Ok(self
            .active_channels
            .values()
            .find_map(|cs| cs.get(id).map(|c| *c)))
    }

    async fn status_of(
        &self,
        user_id: &Key<User>,
    ) -> AppResult<HashMap<Key<Channel>, ChannelStatus>> {
        Ok(self
            .active_channels
            .get(user_id)
            .map(|m| m.clone())
            .unwrap_or(Default::default()))
    }

    async fn start_channels(&self) {
    async fn is_running(&self, id: &Key<Channel>) -> AppResult<bool> {
        Ok(self
            .active_channels
            .iter()
            .any(|(_, chs)| chs.contains_key(id)))
    }
        todo!()
    }

    async fn stop_channels(&self) {
        todo!()
    }
}
