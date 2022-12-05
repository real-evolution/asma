use axum::{routing::get, Router};

pub async fn echo(body: String) -> String {
    body
}

pub fn routes() -> Router {
    Router::new().route("/echo", get(echo).post(echo))
}
