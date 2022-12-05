pub mod add;
pub mod dtos;
pub mod remove;
pub mod update;
pub mod view;

use axum::{routing::*, Router};
use driver_web_common::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(view::get_all).post(add::add))
        .route(
            "/:role_id",
            get(view::get_by_id)
                .delete(remove::remove)
                .patch(update::update),
        )
        .route("/accounts", post(add::add_to).delete(remove::remove_from))
        .route(
            "/permissions/:permission_id",
            post(add::add_permission).delete(remove::remove_permission),
        )
}
