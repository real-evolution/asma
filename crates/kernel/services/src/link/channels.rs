#[async_trait::async_trait]
pub trait ChannelsService: Send + Sync {
    async fn start_channels(&self);
    async fn stop_channels(&self);
}
