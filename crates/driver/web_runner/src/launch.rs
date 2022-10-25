use std::{net::SocketAddr, sync::Arc};

use axum::Extension;
use driver_web_common::di;
use driver_web_rest::app::make_app;
use kernel_services::config::ConfigService;
use tower_http::trace::TraceLayer;

use crate::config;

pub(super) async fn launch_with_di() -> anyhow::Result<()> {
    let base_di = di::base_services_module()?;
    let config_svc: &dyn ConfigService = base_di.resolve_ref();

    config::log::configure_logger_with(config_svc)
        .expect("could not setup logging");

    let bind_addr = config::web::get_bind_address(config_svc)?;

    debug!("creating dependency-injection container");
    let di = di::build_di(base_di).await?;

    Ok(launch(di, bind_addr).await?)
}

async fn launch(
    di: Arc<dyn di::DI>,
    bind_addr: SocketAddr,
) -> anyhow::Result<()> {
    info!("running server on: {bind_addr}");

    Ok(axum::Server::try_bind(&bind_addr)?
        .serve(
            make_app()
                .layer(Extension(di))
                .layer(TraceLayer::new_for_http())
                .into_make_service(),
        )
        .await?)
}
