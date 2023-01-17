mod add;
mod dtos;
mod view;

use aide::axum::{routing::get, ApiRouter};
use driver_web_common::state::AppState;

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get(view::get_all).post(add::add))
        .api_route("/:channel_id", get(view::get_by_id))
}
