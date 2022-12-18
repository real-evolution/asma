use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::StreamExt;
use kernel_entities::entities::link::Channel;
use kernel_services::error::AppResult;
use tokio_util::sync::CancellationToken;

use super::handlers::{self, ChannelHandler};

#[derive(Clone)]
pub(super) struct ChannelState {
    handler: Arc<dyn ChannelHandler>,
    cancellation: CancellationToken,
    started_at: DateTime<Utc>,
    channel: Channel,
}

impl ChannelState {
    pub(super) async fn create(channel: Channel) -> AppResult<Self> {
        let state = Self {
            cancellation: CancellationToken::new(),
            handler: handlers::create_handler(&channel),
            started_at: Utc::now(),
            channel,
        };

        state.start().await?;

        Ok(state)
    }

    pub(super) async fn stop(&self) -> AppResult<()> {
        self.cancellation.cancel();

        Ok(())
    }

    async fn start(&self) -> AppResult<()> {
        let handler = self.handler.clone();
        let cancellation = self.cancellation.clone();

        tokio::spawn(async move {
            let mut updates = handler.updates().await;

            while !cancellation.is_cancelled() {
                tokio::select! {
                    Some(update) = updates.next() => {
                        info!("got an update: {update:#?}");
                    }

                _ = cancellation.cancelled() => {
                        debug!("handler stopped due to cancellation");
                        break;
                    }

                else => {
                        debug!("handler stopped due to unexpected error");
                        break;
                    }
                };
            }

            if !cancellation.is_cancelled() {
                cancellation.cancel();
            }
        });

        Ok(())
    }

    pub(super) fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub(super) fn channel(&self) -> &Channel {
        &self.channel
    }
}
