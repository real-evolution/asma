use std::str::FromStr;

use derive_more::Constructor;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use futures::StreamExt;
use kernel_entities::{
    entities::{
        auth::{Action, KnownRoles, Resource},
        comm::{Message, MessageDirection},
    },
    traits::Key,
};
use kernel_services::{
    self,
    comm::chats::{ChatEventKind, ChatsService},
};
use tonic::{codegen::BoxStream, Request, Response, Status};
use tracing::warn;

use crate::{
    proto::{
        models,
        services::{
            self,
            chats_server::Chats,
            MessageAddedEvent,
            WatchResponse,
        },
        ProtoResult,
    },
    util::{
        auth::token::RequestExt,
        convert::TryConvertInto,
        error::IntoStatusResult,
    },
};

#[derive(Constructor)]
pub(crate) struct GrpcChatsService {
    state: AppState,
}

#[tonic::async_trait]
impl Chats for GrpcChatsService {
    type GetMessagesStream = BoxStream<models::Message>;
    type WatchStream = BoxStream<WatchResponse>;

    async fn get_messages(
        &self,
        req: Request<services::GetMessagesRequest>,
    ) -> ProtoResult<Response<Self::GetMessagesStream>> {
        let auth = req.auth(self.state.config.clone())?;
        let req = req.into_inner();

        let chat_id = req.chat_id.map(|i| i.value).try_convert()?;
        let pagination = req.pagination.try_convert()?;

        auth.can(&[
            (Resource::Chat, Action::View),
            (Resource::Message, Action::View),
        ])?;

        let chat = self
            .state
            .docs
            .chats()
            .get(&chat_id)
            .await
            .into_status_result()?;

        auth.of(&chat.user_id)
            .or_else(|_| auth.in_role(KnownRoles::Admin))?;

        let messages = self
            .state
            .docs
            .messages()
            .get_paginated_of(
                &chat_id,
                &pagination.before,
                pagination.page_size,
            )
            .await
            .into_status_result()?
            .into_iter()
            .map(|m| {
                let direction: models::message::Direction = m.direction.into();

                Ok(crate::proto::models::Message {
                    id: Some(models::message::Id {
                        value: m.id.to_string(),
                    }),
                    text: m.text.unwrap_or_default(),
                    direction: direction.into(),
                    user_id: Some(models::user::Id {
                        value: m.user_id.to_string(),
                    }),
                    chat_id: Some(models::chat::Id {
                        value: m.chat_id.to_string(),
                    }),
                    instance_id: Some(models::instance::Id {
                        value: m.instance_id.to_string(),
                    }),
                    delivered_at: Some(m.delivered_at.into()),
                    seen_at: m.seen_at.map(Into::into),
                    deleted_at_at: m.deleted_at.map(Into::into),
                    created_at: Some(m.created_at.into()),
                    updated_at: Some(m.updated_at.into()),
                })
            });

        Ok(Response::new(tokio_stream::iter(messages).boxed()))
    }

    async fn watch(
        &self,
        req: Request<models::user::Id>,
    ) -> ProtoResult<Response<Self::WatchStream>> {
        let auth = req.auth(self.state.config.clone())?;

        let key_value = req.into_inner().value;
        let Ok(user_id) = Key::from_str(&key_value) else {
            error!("user #{} sent an invalid key: {}", auth.user_id, key_value);
            return Err(Status::invalid_argument("invalid key format"));
        };

        auth.can(&[
            (Resource::Chat, Action::View),
            (Resource::Message, Action::View),
        ])?
        .of(&user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

        let mut stream = match self.state.chats.watch_user_chats(&user_id).await
        {
            | Ok(stream) => stream,
            | Err(err) => {
                error!("an error occured during updates subscription: {err}");

                return Err(Status::invalid_argument(
                    "could not subscribe to updates",
                ));
            }
        };

        let output = async_stream::stream! {
            while let Some(event) = stream.next().await {

                let event = match event {
                    Ok(event) => event,
                    Err(err) => {
                        warn!("an error occured while reading event: {err:#?}");

                        yield Err(Status::internal("could not read event"));

                        return ();
                    }
                };

                match event.kind {
                    | ChatEventKind::MessageAdded {
                        id,
                        text,
                        instance_id,
                        direction,
                        created_at,
                    } => {
                        let direction: models::message::Direction =
                            direction.into();

                        yield Ok(WatchResponse {
                            message_added: Some(MessageAddedEvent {
                                id: Some(models::message::Id {
                                    value: id.to_string(),
                                }),
                                chat_id: Some(models::chat::Id {
                                    value: event.chat_id.to_string(),
                                }),
                                instance_id: Some(models::instance::Id {
                                    value: instance_id.to_string(),
                                }),
                                direction: direction.into(),
                                text: text.unwrap_or_default(),
                                created_at: Some(created_at.into()),
                            }),
                        });
                    }
                }
            }

            return ()
        }
        .boxed();

        Ok(Response::new(output))
    }

    async fn send(
        &self,
        req: Request<services::SendMessageRequest>,
    ) -> ProtoResult<Response<()>> {
        let auth = req.auth(self.state.config.clone())?;
        let req = req.into_inner();

        let chat_id = req.chat_id.map(|i| i.value).try_convert()?;

        auth.can(&[
            (Resource::Chat, Action::View),
            (Resource::Message, Action::View),
        ])?;

        let chat = self
            .state
            .docs
            .chats()
            .get(&chat_id)
            .await
            .into_status_result()?;

        auth.of(&chat.user_id)
            .or_else(|_| auth.in_role(KnownRoles::Admin))?;

        self.state
            .chats
            .send_message(&chat_id, req.text)
            .await
            .into_status_result()?;

        Ok(Response::new(()))
    }
}

impl From<MessageDirection> for models::message::Direction {
    fn from(value: MessageDirection) -> Self {
        match value {
            | MessageDirection::Incoming => Self::Incoming,
            | MessageDirection::Outgoing => Self::Outgoing,
        }
    }
}

impl From<Message> for models::Message {
    fn from(m: Message) -> Self {
        let direction: models::message::Direction = m.direction.into();

        Self {
            id: Some(models::message::Id {
                value: m.id.to_string(),
            }),
            text: m.text.unwrap_or_default(),
            direction: direction.into(),
            user_id: Some(models::user::Id {
                value: m.user_id.to_string(),
            }),
            chat_id: Some(models::chat::Id {
                value: m.chat_id.to_string(),
            }),
            instance_id: Some(models::instance::Id {
                value: m.instance_id.to_string(),
            }),
            delivered_at: Some(m.delivered_at.into()),
            seen_at: m.seen_at.map(Into::into),
            deleted_at_at: m.deleted_at.map(Into::into),
            created_at: Some(m.created_at.into()),
            updated_at: Some(m.updated_at.into()),
        }
    }
}
