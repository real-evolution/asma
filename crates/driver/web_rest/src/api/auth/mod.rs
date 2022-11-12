mod config;
mod signin;
mod util;
pub mod templates;

use axum::{routing::post, Extension, Router};
use kernel_services::{config::ConfigService, error::AppResult, get_config};
pub use signin::*;

use self::config::{ApiTokenConfig, API_TOKEN_CONFIG_SECTION};

pub fn routes(config_svc: &dyn ConfigService) -> AppResult<Router> {
    let conf =
        get_config!(config_svc, API_TOKEN_CONFIG_SECTION => ApiTokenConfig)?;

    Ok(Router::new()
        .route("/signin", post(signin))
        .layer(Extension(conf)))
}
