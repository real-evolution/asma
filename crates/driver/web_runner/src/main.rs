mod config;

use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use common_validation::validators::traits::ValidateString;
use driver_web_common::di::{build_di, shaku::HasComponent, RootModule};
use driver_web_rest::app::make_app;
use kernel_services::config::ConfigService;

use axum::Extension;

#[tokio::main]
async fn main() {
    let di = build_di().unwrap();
    let (ip, port) = get_launch_config(&di).unwrap();

    let rest_app = make_app().layer(Extension(di));

    axum::Server::bind(&SocketAddr::new(ip, port))
        .serve(rest_app.into_make_service())
        .await
        .unwrap();
}

fn get_launch_config(di: &Arc<RootModule>) -> anyhow::Result<(IpAddr, u16)> {
    let config_svc: &dyn ConfigService = di.resolve_ref();

    let config: config::WebConfig = erased_serde::deserialize(
        &mut config_svc.get_section(config::WEB_CONFIG_SECTION)?,
    )?;

    let addr =
        common_validation::parse::IpEndpoint::parse_str(config.listen_addr)?;

    Ok((addr.ip, addr.port.unwrap_or(config::WEB_DEFAULT_PORT)))
}
