use crate::api;

use axum::Router;

pub fn make_app() -> Router {
    api::api_routes()
}
