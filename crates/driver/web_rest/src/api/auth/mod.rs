pub mod dtos;
pub mod signin;

use axum::{routing::post, Router};
use driver_web_common::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/signin", post(signin::signin))
}
