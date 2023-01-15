use std::str::FromStr;

use driver_web_common::state::AppState;
use futures::StreamExt;
use kernel_entities::{entities::comm::MessageDirection, traits::Key};
use kernel_services::{
    self,
    comm::chats::{ChatEventKind, ChatsService},
};
use tonic::{codegen::BoxStream, Request, Response, Status};
use tracing::warn;

use crate::proto::{
    models,
    services::{chats_server::Chats, MessageAddedEvent, WatchResponse},
    ProtoResult,
};

pub(super) struct ChatsServiceImpl {
    state: AppState,
}

#[tonic::async_trait]
impl Chats for ChatsServiceImpl {
    type WatchStream = BoxStream<WatchResponse>;

    async fn watch(
        &self,
        req: Request<models::user::Id>,
    ) -> ProtoResult<Response<Self::WatchStream>> {
        let Ok(user_id) = Key::from_str(&req.into_inner().value) else {
            return Err(Status::invalid_argument("invalid key format"));
        };

        let Ok(mut stream) = self.state.chats.watch_user_chats(&user_id).await else {
            return Err(Status::invalid_argument("could not subscribe to updates"));
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
                                created_at: Some(prost_types::Timestamp {
                                    seconds: created_at.timestamp(),
                                    nanos: created_at.timestamp_nanos() as i32,
                                }),
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
}

impl From<MessageDirection> for models::message::Direction {
    fn from(value: MessageDirection) -> Self {
        match value {
            | MessageDirection::Incoming => Self::Incoming,
            | MessageDirection::Outgoing => Self::Outgoing,
        }
    }
}
