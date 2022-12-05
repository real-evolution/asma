use axum::Router;
use driver_web_common::state::AppState;
use kernel_services::error::AppResult;

use crate::api;

pub fn make_app() -> AppResult<Router<AppState>> {
    Ok(Router::new().nest("/api", api::api_routes()?))
}
