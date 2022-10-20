use crate::api;

use axum::Router;

pub fn make_app() -> Router {
    Router::new().nest("/api", api::api_routes())
}
