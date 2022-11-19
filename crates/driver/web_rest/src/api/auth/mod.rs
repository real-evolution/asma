pub mod signin;
pub mod dtos;

use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/signin", post(signin::signin))
}
