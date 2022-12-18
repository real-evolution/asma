use std::net::SocketAddr;

use driver_web_common::state::create_state;
use driver_web_rest::app::make_app;
use tokio_stream::StreamExt;
use tower_http::trace::TraceLayer;

use crate::config;

pub(super) async fn launch() -> anyhow::Result<()> {
    let state = create_state().await?;

    config::log::configure_logger_with(&*state.config)?;

    // start channels
    {
        error!("starting channels");

        let mut channels = state.channels.start_channels();

        while let Some(res) = channels.next().await {
            if let Err(err) = res {
                warn!("error starting channel: {err}")
            }
        }
    }

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
