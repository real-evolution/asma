use tonic::{codegen::BoxStream, Request, Response};

use crate::proto::{
    models,
    services::{chats_server::Chats, WatchMessagesRequest},
    ProtoResult,
};

#[derive(Debug)]
pub(super) struct ChatsService;

#[async_trait::async_trait]
impl Chats for ChatsService {
    type WatchMessagesStream = BoxStream<models::Message>;

    async fn watch_messages(
        &self,
        _req: Request<WatchMessagesRequest>,
    ) -> ProtoResult<Response<Self::WatchMessagesStream>> {
        todo!()
    }
}
