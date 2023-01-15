use driver_web_common::state::AppState;
use tonic::transport::{server::Router, Server};

use crate::{
    proto::services::chats_server::ChatsServer,
    services::GrpcChatsService,
};

pub fn make_grpc_app(state: AppState) -> Router {
    Server::builder()
        .add_service(ChatsServer::new(GrpcChatsService::new(state.clone())))
}
