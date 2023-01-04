use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::StreamExt;
use kernel_entities::entities::link::Channel;
use kernel_services::error::AppResult;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::channel_stream::ChannelStream;

pub(super) struct ChannelState {
    stream: Arc<dyn ChannelStream>,
    cancellation: CancellationToken,
    started_at: DateTime<Utc>,
    channel: Channel,
    task: JoinHandle<()>,
}

impl ChannelState {
    pub(super) async fn spawn(channel: Channel) -> AppResult<Self> {
        let handler = handlers::create_handler(&channel);
        let cancellation = CancellationToken::new();

        let (h, c) = (handler.clone(), cancellation.clone());

        let task = tokio::spawn(async move {
            let mut updates = h.updates().await;

            while !c.is_cancelled() {
                tokio::select! {
                    Some(update) = updates.next() => {
                        info!("got an update: {update:#?}");
                    }

                _ = c.cancelled() => {
                        debug!("handler stopped due to cancellation");
                        break;
                    }

                else => {
                        debug!("handler stopped due to unexpected error");
                        break;
                    }
                };
            }

            if !c.is_cancelled() {
                c.cancel();
            }
        });

        Ok(Self {
            _handler: handler,
            cancellation,
            channel,
            task,
            started_at: Utc::now(),
        })
    }

    pub(super) async fn stop(self) -> AppResult<()> {
        self.cancellation.cancel();
        self.task.await.map_err(anyhow::Error::new)?;

        Ok(())
    }

    pub(super) fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub(super) fn channel(&self) -> &Channel {
        &self.channel
    }
}
