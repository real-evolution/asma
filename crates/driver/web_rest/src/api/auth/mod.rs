pub mod signin;

use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/signin", post(signin::signin))
}
