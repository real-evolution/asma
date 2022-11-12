use axum::{Extension, Router};
use kernel_services::{config::ConfigService, error::AppResult, get_config};

use crate::{
    api,
    config::{ApiConfig, API_CONFIG_SECTION},
    doc,
};

pub fn make_app(config_svc: &dyn ConfigService) -> AppResult<Router> {
    let conf = get_config!(config_svc, API_CONFIG_SECTION => ApiConfig)?;

    Ok(Router::new()
        .nest("/api", api::api_routes()?)
        .layer(Extension(conf))
        .merge(doc::create_swagger_ui()))
}
