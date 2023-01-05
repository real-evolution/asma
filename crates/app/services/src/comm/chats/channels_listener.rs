use std::sync::Arc;

use kernel_services::{
    error::{AppError, AppResult},
    link::channels::{ChannelPipe, ChannelsService, OutgoingChannelUpdate},
};
use tokio::{
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};
use tokio_stream::StreamExt;

pub(super) struct ChatsChannelsListener<IPC> {
    ipc: Arc<IPC>,
    outgoing_tx: UnboundedSender<OutgoingChannelUpdate>,
    outgoing_task: JoinHandle<()>,
    incoming_task: JoinHandle<()>,
}

impl<IPC> ChatsChannelsListener<IPC> {
    pub(super) async fn create(
        ipc: Arc<IPC>,
        channels_svc: Arc<dyn ChannelsService>,
    ) -> AppResult<Self> {
        let ChannelPipe {
            outgoing: _,
            incoming,
        } = channels_svc.get_pipe_of_all().await?;

        let (outgoing_tx, mut outgoing_rx) =
            mpsc::unbounded_channel::<OutgoingChannelUpdate>();

        let outgoing_task = tokio::spawn(async move {
            while let Some(update) = outgoing_rx.recv().await {
                let Ok(channel_pipe) = channels_svc
                    .get_pipe_of(&update.user_id, Some(&update.channel_id))
                    .await else {
                    error!("could not get pipe of channel #{} of user #{}", &update.channel_id, &update.user_id);
                    continue;
                };

                channel_pipe
                    .outgoing
                    .publish(None, &update)
                    .await
                    .unwrap_or_else(|err| {
                        error!("could not publish channel update: {err:#?}");
                    });
            }
        });

        let incoming_task = tokio::spawn(async move {
            let mut incoming_stream = match incoming.subscribe(None).await {
                | Ok(stream) => stream,
                | Err(err) => {
                    error!("could not subscribe to channels stream: {err:#?}");
                    return;
                }
            };

            while let Some(update) = incoming_stream.next().await {
                let update = match update {
                    | Ok(update) => update,
                    | Err(err) => {
                        warn!(
                            "failed to read channel update from IPC: {err:#?}"
                        );
                        continue;
                    }
                };

                info!("");
                info!("=========================================");
                info!(
                    "Got update from channel #{} of user #{}:",
                    &update.channel_id, &update.user_id
                );
                info!("");
                info!("{:#?}", &update.kind);
                info!("=========================================");
                info!("");
            }
        });

        Ok(Self {
            ipc,
            outgoing_tx,
            outgoing_task,
            incoming_task,
        })
    }

    pub(super) fn enqueue_update(
        &self,
        update: OutgoingChannelUpdate,
    ) -> AppResult<()> {
        self.outgoing_tx
            .send(update)
            .map_err(|err| AppError::Unknown(err.into()))
    }
}
