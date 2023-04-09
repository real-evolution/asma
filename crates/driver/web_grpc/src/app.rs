use driver_web_common::state::AppState;
use tonic::transport::{server::Router, Server};

use crate::{
    proto::services::{chats_server::ChatsServer, stats_server::StatsServer},
    services::{GrpcChatsService, GrpcStatsService},
};

pub fn add_grpc_services<const ENABLE_WEB: bool, T>(
    mut server: Server<T>,
    state: AppState,
) -> Router<T>
where
    T: Clone,
{
    if ENABLE_WEB {
        server
            .accept_http1(true)
            .add_service(tonic_web::enable(ChatsServer::new(
                GrpcChatsService::new(state.clone()),
            )))
            .add_service(tonic_web::enable(StatsServer::new(
                GrpcStatsService::new(state),
            )))
    } else {
        server
            .add_service(ChatsServer::new(GrpcChatsService::new(state.clone())))
            .add_service(StatsServer::new(GrpcStatsService::new(state)))
    }
}
