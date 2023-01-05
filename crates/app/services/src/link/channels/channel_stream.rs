use kernel_services::{
    error::AppResult,
    link::channels::{IncomingChannelUpdateKind, OutgoingChannelUpdateKind},
};

#[async_trait::async_trait]
pub(super) trait ChannelStream: Send + Sync {
    async fn recv(&self) -> AppResult<IncomingChannelUpdateKind>;
    async fn send(&self, update: OutgoingChannelUpdateKind) -> AppResult<()>;
}
