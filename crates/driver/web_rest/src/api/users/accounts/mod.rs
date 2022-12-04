pub mod view;
pub mod remove;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all_of))
        .route("/:account_id", get(view::get_of_by_id))
}
