pub mod dtos;
pub mod view;

use axum::{routing::*, Router};
use driver_web_common::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(view::get_all))
        .route("/:channel_id", get(view::get_by_id))
}
