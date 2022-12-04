pub mod dtos;
pub mod remove;
pub mod update;
pub mod view;

use axum::{
    routing::{get, put},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all_of))
        .route("/:account_id", get(view::get_of_by_id))
        .route("/password", put(update::update_password))
}
