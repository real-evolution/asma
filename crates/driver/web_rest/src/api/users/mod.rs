pub mod dtos;
pub mod view;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new().route("/:user_id", get(view::get_by_id))
}
