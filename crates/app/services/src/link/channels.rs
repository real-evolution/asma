use std::sync::Arc;

use kernel_repositories::DataStore;
use kernel_services::link::channels::ChannelsService;

pub struct AppChannelsService {
    _data: Arc<dyn DataStore>,
}

#[async_trait::async_trait]
impl ChannelsService for AppChannelsService {
    async fn start_channels(&self) {
        todo!()
    }

    async fn stop_channels(&self) {
        todo!()
    }
}
