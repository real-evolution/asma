use std::net::SocketAddr;

use driver_web_common::state::create_state;
use driver_web_rest::app::make_app;
use tower_http::trace::TraceLayer;

use crate::config;

pub(super) async fn launch() -> anyhow::Result<()> {
    let state = create_state().await?;

    config::log::configure_logger_with(&*state.config)?;

    let bind_addr = config::web::get_bind_address(&*state.config)?;
    info!("running server on: {bind_addr}");

    Ok(axum::Server::try_bind(&bind_addr)?
        .serve(
            make_app()?
                .layer(TraceLayer::new_for_http())
                .with_state(state)
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?)
}
