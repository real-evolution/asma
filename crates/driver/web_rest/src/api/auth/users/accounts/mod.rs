mod add;
mod remove;
mod update;
mod view;

pub mod dtos;

use aide::axum::{
    routing::{get, put},
    ApiRouter,
};
use driver_web_common::state::AppState;

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get(view::get_all).post(add::add))
        .api_route("/:account_id", get(view::get_by_id).delete(remove::remove))
        .api_route("/:account_id/roles", get(view::get_roles_and_permissions))
        .api_route("/:account_id/password", put(update::update_password))
}
