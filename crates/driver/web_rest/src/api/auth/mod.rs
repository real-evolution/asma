mod signin;

use axum::{routing::post, Router};
use kernel_services::error::AppResult;
pub use signin::*;

pub fn routes() -> AppResult<Router> {
    Ok(Router::new().route("/signin", post(signin)))
}
