pub mod auth;
pub mod diag;
pub mod dtos;
pub mod link;
pub mod roles;
pub mod setup;
pub mod users;

use axum::Router;
use kernel_services::error::AppResult;

pub fn api_routes() -> AppResult<Router> {
    debug!("creating router with Swagger/OpenAPI support");

    Ok(Router::new()
        .nest("/diag", diag::routes())
        .nest("/setup", setup::routes())
        .nest("/auth", auth::routes())
        .nest("/roles", roles::routes())
        .nest("/users", users::routes())
        .nest("/link", link::routes()))
}
