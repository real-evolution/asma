use std::sync::Arc;

use crate::config::*;

use driver_web_common::di::*;
use driver_web_rest::app::make_app;

use axum::Extension;

pub async fn launch(di: Arc<DI>) -> anyhow::Result<()> {
    let bind_addr = web::get_bind_address(di.resolve_ref())?;
    let server = axum::Server::try_bind(&bind_addr)?;

    info!("running server on: {bind_addr}");

    Ok(server
        .serve(make_app().layer(Extension(di)).into_make_service())
        .await?)
}
