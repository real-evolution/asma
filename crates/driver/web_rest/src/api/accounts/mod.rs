pub mod add;
pub mod dtos;
pub mod remove;
pub mod update;
pub mod view;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(view::get_all))
        .route(
            "/:account_id",
            get(view::get_by_id).post(add::add).delete(remove::remove),
        )
        .route("/password", put(update::update_password))
}
