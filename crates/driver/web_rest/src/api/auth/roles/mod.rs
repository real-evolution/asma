mod add;
mod dtos;
mod remove;
mod update;
mod view;

use aide::axum::{
    routing::{delete, get, post},
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
        .api_route(
            "/:role_id/accounts",
            get(view::get_accounts)
                .post(add::add_to)
                .delete(remove::remove_from),
        )
        .api_route("/:role_id/permissions", post(add::add_permission))
        .api_route(
            "/:role_id/permissions/:permission_id",
            delete(remove::remove_permission),
        )
}
