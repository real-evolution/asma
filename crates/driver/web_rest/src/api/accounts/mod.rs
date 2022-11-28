pub mod add;
pub mod dtos;
pub mod view;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all))
        .route("/:account_id", get(view::get_by_id).post(add::add))
}
