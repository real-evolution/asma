use axum::{routing::get, Router};

async fn echo(body: String) -> String {
    body
}

pub fn diag_routes() -> Router {
    Router::new().route("/echo", get(echo).post(echo))
}
