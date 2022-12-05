pub mod auth;
pub mod diag;
pub mod dtos;
pub mod link;
pub mod roles;
pub mod setup;
pub mod users;

use axum::Router;
use driver_web_common::state::AppState;
use kernel_services::error::AppResult;

pub fn api_routes() -> AppResult<Router<AppState>> {
    debug!("creating api router");

    Ok(Router::<AppState>::new()
        .nest("/diag", diag::routes())
        .nest("/setup", setup::routes())
        .nest("/auth", auth::routes())
        .nest("/roles", roles::routes())
        .nest("/users", users::routes())
        .nest("/link", link::routes()))
}
