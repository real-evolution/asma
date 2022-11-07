pub mod auth;
pub mod diag;

use axum::Router;
use kernel_services::{config::ConfigService, error::AppResult};

pub fn api_routes(config_svc: &dyn ConfigService) -> AppResult<Router> {
    debug!("creating router with Swagger/OpenAPI support");

    Ok(Router::new()
        .nest("/diag", diag::routes())
        .nest("/auth", auth::routes(config_svc)?))
}
