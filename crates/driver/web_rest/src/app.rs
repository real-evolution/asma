use crate::{api, doc};

use axum::Router;

pub fn make_app() -> Router {
    Router::new()
        .nest("/api", api::api_routes())
        .merge(doc::create_swagger_ui())
}
