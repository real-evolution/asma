use axum::{routing::get, Router};

pub fn make_router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> &'static str {
    "Hello, World!"
}
