pub mod dtos;
pub mod view;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all))
        .route("/:channel_id", get(view::get_by_id))
}
