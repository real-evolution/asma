use derive_more::Constructor;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use futures::{StreamExt, TryStreamExt};
use kernel_entities::entities::{
    auth::{Action, KnownRoles, Resource},
    comm::{Chat, ChatState, Message, MessageDirection},
};
use kernel_services::{
    self,
    comm::chats::{ChatEventKind, ChatsService},
};
use tonic::{codegen::BoxStream, Request, Response};

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
        auth::token::{GrpcAuthToken, RequestExt},
        convert::TryConvertInto,
        error::{IntoStatus, IntoStatusResult},
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
        let services::GetMessagesRequest {
            chat_id,
            pagination,
        } = req.into_inner();

        auth.can(&[(Resource::Message, Action::View)])?;

        let pagination = pagination.try_convert()?;
        let chat = self.get_chat_by_id(&auth, chat_id).await?;

        let messages = self
            .state
            .docs
            .messages()
            .get_paginated_of(
                &chat.id,
                &pagination.before,
                pagination.page_size,
            )
            .await
            .into_status_result()?
            .into_iter()
            .map(|m| Ok(m.into()));

        Ok(Response::new(tokio_stream::iter(messages).boxed()))
    }

    async fn watch(
        &self,
        req: Request<models::user::Id>,
    ) -> ProtoResult<Response<Self::WatchStream>> {
        let auth = req.auth(self.state.config.clone())?;
        let user_id = req.into_inner().value.try_convert()?;

        auth.can(&[
            (Resource::Chat, Action::View),
            (Resource::Message, Action::View),
        ])?
        .of(&user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

        let mut stream = self
            .state
            .chats
            .watch_user_chats(&user_id)
            .await
            .into_status_result()?
            .map_err(IntoStatus::into_status);

        let output = async_stream::stream! {
            while let Some(event) = stream.try_next().await? {
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
                                id: Some(id.into()),
                                chat_id: Some(event.chat_id.into()),
                                instance_id: Some(instance_id.into()),
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
        let services::SendMessageRequest { chat_id, text } = req.into_inner();

        auth.can(&[(Resource::Message, Action::Add)])?;

        let chat = self.get_chat_by_id(&auth, chat_id).await?;

        self.state
            .chats
            .send_message(&chat.id, text)
            .await
            .into_status_result()?;

        Ok(Response::new(()))
    }

    async fn get_chat(
        &self,
        req: Request<models::chat::Id>,
    ) -> ProtoResult<Response<models::Chat>> {
        let auth = req.auth(self.state.config.clone())?;
        let chat = self.get_chat_by_id(&auth, Some(req.into_inner())).await?;

        Ok(Response::new(chat.into()))
    }
}

impl GrpcChatsService {
    async fn get_chat_by_id(
        &self,
        auth: &GrpcAuthToken,
        chat_id: Option<models::chat::Id>,
    ) -> ProtoResult<Chat> {
        auth.can(&[(Resource::Chat, Action::View)])?;

        let chat = self
            .state
            .docs
            .chats()
            .get(&chat_id.try_convert()?)
            .await
            .into_status_result()?;

        auth.of(&chat.user_id)
            .or_else(|_| auth.in_role(KnownRoles::Admin))?;

        Ok(chat)
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

impl From<ChatState> for models::chat::State {
    fn from(value: ChatState) -> Self {
        match value {
            | ChatState::Active => Self::Active,
            | ChatState::Archived => Self::Archived,
            | ChatState::Closed => Self::Closed,
        }
    }
}

impl From<Message> for models::Message {
    fn from(value: Message) -> Self {
        let direction: models::message::Direction = value.direction.into();

        Self {
            id: Some(value.id.into()),
            text: value.text.unwrap_or_default(),
            direction: direction.into(),
            user_id: Some(value.user_id.into()),
            chat_id: Some(value.chat_id.into()),
            instance_id: Some(value.instance_id.into()),
            delivered_at: Some(value.delivered_at.into()),
            seen_at: value.seen_at.map(Into::into),
            deleted_at_at: value.deleted_at.map(Into::into),
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
        }
    }
}

impl From<Chat> for models::Chat {
    fn from(value: Chat) -> Self {
        let state: models::chat::State = value.state.into();

        Self {
            id: Some(value.id.into()),
            label: value.label,
            state: state.into(),
            user_id: Some(value.user_id.into()),
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
        }
    }
}
