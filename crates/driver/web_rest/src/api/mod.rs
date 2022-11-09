pub mod auth;
pub mod diag;
pub mod setup;

use axum::Router;
use kernel_services::{config::ConfigService, error::AppResult};

pub fn api_routes(config_svc: &dyn ConfigService) -> AppResult<Router> {
    debug!("creating router with Swagger/OpenAPI support");

    Ok(Router::new()
        .nest("/diag", diag::routes())
        .nest("/setup", setup::routes())
        .nest("/auth", auth::routes(config_svc)?))
}
