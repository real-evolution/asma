pub mod dtos;
pub mod remove;
pub mod view;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all_of))
        .route("/:account_id", get(view::get_of_by_id))
}
