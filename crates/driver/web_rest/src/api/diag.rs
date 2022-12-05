use axum::{routing::get, Router};
use driver_web_common::state::AppState;

pub async fn echo(body: String) -> String {
    body
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/echo", get(echo).post(echo))
}
