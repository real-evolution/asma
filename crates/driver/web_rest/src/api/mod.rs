pub mod auth;
pub mod comm;
pub mod diag;
pub mod dtos;
pub mod link;
pub mod setup;

use aide::axum::ApiRouter;
use driver_web_common::state::AppState;
use kernel_services::error::AppResult;

pub fn api_routes() -> AppResult<ApiRouter<AppState>> {
    debug!("creating api router");

    Ok(ApiRouter::<AppState>::new()
        .nest("/diag", diag::routes())
        .nest("/setup", setup::routes())
        .nest("/auth", auth::routes())
        .nest("/link", link::routes()))
}
