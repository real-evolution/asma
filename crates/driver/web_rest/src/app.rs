use axum::Router;
use kernel_services::{config::ConfigService, error::AppResult};

use crate::{api, doc};

pub fn make_app(config_svc: &dyn ConfigService) -> AppResult<Router> {
    Ok(Router::new()
        .nest("/api", api::api_routes(config_svc)?)
        .merge(doc::create_swagger_ui()))
}
