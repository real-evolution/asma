pub mod accounts;
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
            "/:user_id",
            get(view::get_by_id)
                .delete(remove::remove)
                .patch(update::update),
        )
        .nest("/:user_id/accounts", accounts::routes())
}
