#[macro_use]
extern crate common_macros;
#[macro_use]
extern crate tracing;

mod config;
mod launch;

use driver_web_common::di::{build_di, HasComponent};
use kernel_services::config::ConfigService;
use std::process::{ExitCode, Termination};

#[tokio::main]
async fn main() -> impl Termination {
    let di = build_di().expect("could not setup DI");
    let config_svc: &dyn ConfigService = di.resolve_ref();

    config::log::configure_logger_with(config_svc)
        .expect("could not setup logging");

    if let Err(err) = launch::launch(di).await {
        error!("app terminated with error: {}", err);
        return ExitCode::FAILURE;
    }

    info!("app exited normally");
    return ExitCode::FAILURE;
}
