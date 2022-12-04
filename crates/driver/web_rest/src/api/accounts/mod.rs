pub mod add;
pub mod dtos;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new().route("/:account_id", post(add::add))
}
