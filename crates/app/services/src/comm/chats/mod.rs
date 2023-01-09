use std::sync::Arc;

use chrono::Utc;
use kernel_entities::{
    entities::{
        comm::{Chat, MessageDirection},
        link::Instance,
    },
    traits::Key,
};
use kernel_repositories::{comm::InsertMessage, DataStore, DocumentStore};
use kernel_services::{
    comm::chats::ChatsService,
    error::AppResult,
    link::channels::{
        ChannelPipe, ChannelsService, IncomingChannelUpdate,
        OutgoingChannelUpdate, OutgoingChannelUpdateKind,
        OutgoingMessageUpdateKind,
    },
    Service,
};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

pub struct AppChatsService {
    data: Arc<dyn DataStore>,
    docs: Arc<dyn DocumentStore>,
    channels_svc: Arc<dyn ChannelsService>,
    read_task: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

#[async_trait::async_trait]
impl ChatsService for AppChatsService {
    async fn send_message(
        &self,
        chat_id: &Key<Chat>,
        text: String,
    ) -> AppResult<()> {
        let chat = self.docs.chats().get(chat_id).await?;
        let instance =
            self.data.link().instances().get(&chat.instance_id).await?;

        self.send_update(text, chat, instance).await
    }
}

impl AppChatsService {
    pub async fn create(
        data: Arc<dyn DataStore>,
        docs: Arc<dyn DocumentStore>,
        channels_svc: Arc<dyn ChannelsService>,
    ) -> AppResult<Self> {
        Ok(Self {
            data,
            docs,
            channels_svc,
            read_task: Default::default(),
        })
    }

    pub(super) async fn send_update(
        &self,
        text: String,
        chat: Chat,
        instance: Instance,
    ) -> AppResult<()> {
        let ChannelPipe { tx, rx: _ } = self
            .channels_svc
            .get_pipe_of(&chat.user_id, Some(&chat.channel_id))
            .await?;

        tx.publish(
            None,
            &OutgoingChannelUpdate {
                user_id: chat.user_id,
                channel_id: chat.channel_id,
                kind: OutgoingChannelUpdateKind::Message {
                    platform_user_id: instance.platform_identifier,
                    kind: OutgoingMessageUpdateKind::New {
                        content: text.clone(),
                    },
                    timestamp: Utc::now(),
                },
            },
        )
        .await?;

        self.docs
            .messages()
            .create(InsertMessage {
                chat_id: chat.id,
                text: Some(text),
                direction: MessageDirection::Outgoing,
            })
            .await?;

        Ok(())
    }

    async fn handle_incoming(update: IncomingChannelUpdate) -> AppResult<()> {
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

        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for AppChatsService {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        debug!("starting chats inbound listener");

        let ChannelPipe { tx: _, rx } =
            self.channels_svc.get_pipe_of_all().await?;

        let read_task = tokio::spawn(async move {
            let mut stream = match rx.subscribe_manual(None).await {
                | Ok(stream) => stream,
                | Err(err) => {
                    error!("could not acquire updates steream: {err:#?}");
                    return;
                }
            };

            loop {
                let (update, confirm) = match stream.try_next().await {
                    | Ok(update) => match update {
                        | Some(update) => update,
                        | None => {
                            debug!("incoming updates stream has terminated");
                            break;
                        }
                    },
                    | Err(err) => {
                        error!("could not read next update: {err:#?}");
                        continue;
                    }
                };

                if let Err(err) = match Self::handle_incoming(update).await {
                    | Ok(()) => confirm.ack().await,
                    | Err(err) => {
                        error!(
                            "an error occured while handling update: {err:#?}"
                        );
                        confirm.nack(true).await
                    }
                } {
                    error!("could not ack/nack IPC message: {err:#?}");
                };
            }
        });

        *self.read_task.lock().await = Some(read_task);

        Ok(())
    }
}
