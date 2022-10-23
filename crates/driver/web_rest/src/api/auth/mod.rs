mod signin;

pub use signin::*;

use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/signin", post(signin))
}
