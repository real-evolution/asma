pub mod add;
pub mod remove;
pub mod update;
pub mod view;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all).post(add::add))
        .route("/:id", get(view::get_by_id))
        .nest(
            ":role_id/permissions",
            Router::new()
                .route("/", post(add::add_permission))
                .route("/:permission_id", delete(remove::remove_permission)),
        )
}
