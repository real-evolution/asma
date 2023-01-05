#[macro_use]
extern crate common_macros;
#[macro_use]
extern crate tracing;

mod config;
mod launch;

use std::process::{ExitCode, Termination};

#[tokio::main]
async fn main() -> impl Termination {
    if let Err(err) = launch::launch().await {
        error!("app terminated with error: {}", err);
        return ExitCode::FAILURE;
    }

    info!("app exited normally");

    ExitCode::SUCCESS
}
