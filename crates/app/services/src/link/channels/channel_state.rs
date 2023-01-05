use std::sync::Arc;

use chrono::{DateTime, Utc};
use kernel_entities::entities::link::{Channel, ChannelPlatform};
use kernel_services::{error::AppResult, link::channels::ChannelPipe};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use super::{
    channel_stream::ChannelStream,
    telegram::telegram_stream::TelegramStream,
};

pub(super) struct ChannelState {
    channel: Channel,
    stream: Arc<dyn ChannelStream>,
    pipe: ChannelPipe,
    cancellation: CancellationToken,
    started_at: DateTime<Utc>,
}

impl ChannelState {
    pub(super) fn new(channel: Channel, pipe: ChannelPipe) -> AppResult<Self> {
        let stream = match channel.platform {
            | ChannelPlatform::Telegram => TelegramStream::new(&channel)?,
        };

        Ok(Self {
            channel,
            stream,
            pipe,
            cancellation: CancellationToken::new(),
            started_at: Utc::now(),
        })
    }

    pub(super) async fn run(&self) -> AppResult<()> {
        let (stream, pipe, cancellation) = (
            self.stream.clone(),
            self.pipe.clone(),
            self.cancellation.clone(),
        );

        tokio::spawn(async move {
            let Ok(mut outgoing_stream) = pipe.outgoing.subscribe_manual(None).await else {
                error!("could not subscribe to channel IPC pipe");
                return;
            };

            while !cancellation.is_cancelled() {
                tokio::select! {
                    Some(Ok((update, confirm))) = outgoing_stream.next() => {
                        if let Err(err) = match stream.send(update).await {
                            Ok(_) => confirm.ack().await,
                            Err(err) => {
                                warn!("could not send outgoing update: {err:#?}");
                                confirm.nack(true).await
                            }
                        } {
                            error!("failed to send ack/nack: {err:#?}");
                        }
                    },

                    Ok(update) = stream.recv() => {
                        if let Err(err) = pipe.incoming.publish(None, &update).await {
                            warn!("could not publish update: {err:#?}");
                        }
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

            cancellation.cancelled().await;
        });

        Ok(())
    }

    pub(super) async fn stop(self) -> AppResult<()> {
        self.cancellation.cancel();
        Ok(())
    }

    pub(super) fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub(super) fn channel(&self) -> &Channel {
        &self.channel
    }
}
