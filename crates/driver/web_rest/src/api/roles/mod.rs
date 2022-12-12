pub mod add;
pub mod dtos;
pub mod remove;
pub mod update;
pub mod view;

use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use driver_web_common::state::AppState;

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get(view::get_all).post(add::add))
        .api_route(
            "/:role_id",
            get(view::get_by_id)
                .delete(remove::remove)
                .patch(update::update),
        )
        .api_route("/accounts", post(add::add_to).delete(remove::remove_from))
        .api_route(
            "/permissions/:permission_id",
            post(add::add_permission).delete(remove::remove_permission),
        )
}
