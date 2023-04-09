use std::net::SocketAddr;

use driver_web_common::state::{create_state, get_config_service, AppState};
use driver_web_grpc::app::add_grpc_services;
use driver_web_rest::app::make_rest_app;
use kernel_services::config::ConfigService;
use tower_http::trace::TraceLayer;

use crate::config::log::configure_logger_with;
use crate::config::web::{
    GrpcLanuchConfig, LanuchConfig, RestLanuchConfig, WEB_CONFIG_SECTION,
};

pub(super) async fn launch() -> anyhow::Result<()> {
    let config = get_config_service().await?;
    configure_logger_with(&*config)?;

    info!("initializing application");
    let state = create_state(config.clone()).await?;

    info!("starting listeners");
    let lanuch_config: LanuchConfig = config.get_section(WEB_CONFIG_SECTION)?;

    futures::future::join_all(vec![
        tokio::spawn(run_rest_server(state.clone(), lanuch_config.rest)),
        tokio::spawn(run_grpc_server(state, lanuch_config.grpc)),
    ])
    .await;

    Ok(())
}

async fn run_rest_server(
    state: AppState,
    config: RestLanuchConfig,
) -> anyhow::Result<()> {
    info!("running RESTful server on: {}", config.listen_on);

    axum::Server::try_bind(&config.listen_on)?
        .serve(
            make_rest_app()?
                .with_state(state)
                .layer(config.cors.into_layer()?)
                .layer(TraceLayer::new_for_http())
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

    Ok(())
}

async fn run_grpc_server(
    state: AppState,
    config: GrpcLanuchConfig,
) -> anyhow::Result<()> {
    info!("running gRPC server on: {}", config.listen_on);

    let server =
        tonic::transport::Server::builder().layer(config.cors.into_layer()?);

    if config.enable_http1 {
        add_grpc_services::<true, _>(server, state)
            .serve(config.listen_on)
            .await?;
    } else {
        add_grpc_services::<false, _>(server, state)
            .serve(config.listen_on)
            .await?;
    };

    Ok(())
}
