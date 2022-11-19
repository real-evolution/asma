pub mod add;
pub mod remove;
pub mod update;
pub mod view;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(view::get_all).get(view::get_by_id))
}
