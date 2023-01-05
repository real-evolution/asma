use kernel_services::{
    error::AppResult,
    link::channels::{IncomingChannelUpdate, OutgoingChannelUpdate},
};

#[async_trait::async_trait]
pub(super) trait ChannelStream: Send + Sync {
    async fn recv(&self) -> AppResult<IncomingChannelUpdate>;
    async fn send(&self, update: OutgoingChannelUpdate) -> AppResult<()>;
}
