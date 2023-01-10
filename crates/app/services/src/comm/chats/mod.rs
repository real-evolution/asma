use std::sync::Arc;

use chrono::Utc;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, ChatState, MessageDirection},
        link::{Channel, Instance},
    },
    traits::Key,
};
use kernel_repositories::{
    comm::{InsertChat, InsertMessage},
    error::{RepoError, RepoResult},
    link::InsertInstance,
    DataStore, DocumentStore,
};
use kernel_services::{
    comm::chats::ChatsService,
    error::AppResult,
    link::channels::{
        ChannelPipe, ChannelsService, IncomingChannelUpdate,
        IncomingChannelUpdateKind, IncomingMessageUpdateKind,
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

        self.send_update(chat, text).await
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
        chat: Chat,
        text: String,
    ) -> AppResult<()> {
        let instances = self
            .data
            .link()
            .instances()
            .get_members_of(&chat.id)
            .await?;

        for instance in instances {
            let ChannelPipe { tx, rx: _ } = self
                .channels_svc
                .get_pipe_of(&chat.user_id, Some(&instance.channel_id))
                .await?;

            tx.publish(&OutgoingChannelUpdate {
                user_id: chat.user_id.clone(),
                channel_id: instance.channel_id,
                kind: OutgoingChannelUpdateKind::Message {
                    platform_user_id: instance.platform_identifier,
                    kind: OutgoingMessageUpdateKind::New {
                        content: text.clone(),
                    },
                    timestamp: Utc::now(),
                },
            })
            .await?;

            self.docs
                .messages()
                .create(InsertMessage {
                    text: Some(text.clone()),
                    direction: MessageDirection::Outgoing,
                    chat_id: chat.id.clone(),
                    instance_id: instance.id,
                    delivered_at: Utc::now(),
                })
                .await?;
        }

        Ok(())
    }

    async fn handle_incoming(
        &self,
        update: IncomingChannelUpdate,
    ) -> AppResult<()> {
        debug!(
            "Got update from channel #{} of user #{}: {update:#?}",
            &update.channel_id, &update.user_id,
        );

        match update.kind {
            | IncomingChannelUpdateKind::Message {
                platform_user_id,
                kind,
                timestamp,
            } => {
                let instance = self
                    .ensure_instance_created(
                        &update.user_id,
                        &update.channel_id,
                        platform_user_id,
                    )
                    .await?;

                match kind {
                    | IncomingMessageUpdateKind::New { content } => {
                        let message = self
                            .docs
                            .messages()
                            .create(InsertMessage {
                                text: content.clone(),
                                direction: MessageDirection::Incoming,
                                delivered_at: timestamp,
                                chat_id: instance.chat_id,
                                instance_id: instance.id.clone(),
                            })
                            .await?;

                        debug!(
                            "message from instance #{} saved with #{}",
                            instance.id, message.id
                        );
                    }
                }
            }
        };

        Ok(())
    }

    async fn ensure_instance_created(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
        identifier: i64,
    ) -> RepoResult<Instance> {
        let ret = self
            .data
            .link()
            .instances()
            .get_by_platform_identifier(channel_id, identifier)
            .await;

        if let Err(RepoError::NotFound) = ret {
            info!("a new instance was detected on channel #{channel_id}");
            debug!("creating a chat for the new instance");

            let chat = self
                .docs
                .chats()
                .create(InsertChat {
                    label: None,
                    state: ChatState::Active,
                    user_id: user_id.clone(),
                })
                .await?;

            debug!("creating instance record in chat #{}", chat.id);

            let instance = self
                .data
                .link()
                .instances()
                .create(InsertInstance {
                    platform_identifier: identifier,
                    username: None,
                    display_name: None,
                    phone_number: None,
                    chat_id: chat.id.clone(),
                    channel_id: channel_id.clone(),
                })
                .await?;

            return Ok(instance);
        }

        ret
    }
}

#[async_trait::async_trait]
impl Service for AppChatsService {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        debug!("starting chats inbound listener");

        let ChannelPipe { tx: _, rx } =
            self.channels_svc.get_pipe_of_all().await?;

        let this = self.clone();

        *self.clone().read_task.lock().await = Some(tokio::spawn(async move {
            let mut stream = match rx.subscribe_manual().await {
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

                if let Err(err) = match this.handle_incoming(update).await {
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
        }));

        Ok(())
    }
}
