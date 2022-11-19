pub mod add;
pub mod dtos;
pub mod view;
pub mod remove;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all).post(add::add))
        .route("/:user_id", get(view::get_by_id).delete(remove::remove))
}
