use std::net::SocketAddr;

use driver_web_common::state::{create_state, get_config_service, AppState};
use driver_web_grpc::app::make_grpc_app;
use driver_web_rest::app::make_rest_app;
use tower_http::trace::TraceLayer;

use crate::config::{log::configure_logger_with, web::ListenAddressPair};

pub(super) async fn launch() -> anyhow::Result<()> {
    let config = get_config_service().await?;
    configure_logger_with(&*config)?;

    info!("initializing application");
    let state = create_state(config).await?;

    info!("starting listeners");
    let addr_pair = ListenAddressPair::load(&*state.config)?;

    futures::future::join_all(vec![
        tokio::spawn(run_rest_server(state.clone(), addr_pair.rest)),
        tokio::spawn(run_grpc_server(state, addr_pair.grpc)),
    ])
    .await;

    Ok(())
}

async fn run_rest_server(
    state: AppState,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    info!("running RESTful server on: {addr}");

    axum::Server::try_bind(&addr)?
        .serve(
            make_rest_app()?
                .layer(TraceLayer::new_for_http())
                .with_state(state)
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

    Ok(())
}

async fn run_grpc_server(
    state: AppState,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    info!("running gRPC server on: {addr}");

    make_grpc_app(state).serve(addr).await?;

    Ok(())
}
