pub mod add;
pub mod dtos;
pub mod update;

use axum::{routing::*, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/:account_id", get(view::get_by_id).post(add::add))
        .route("/password", put(update::update_password))
}
