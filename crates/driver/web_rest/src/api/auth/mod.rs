pub mod dtos;
pub mod roles;
pub mod signin;

use aide::axum::{routing::post, ApiRouter};
use driver_web_common::state::AppState;

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/signin", post(signin::signin))
        .nest("/roles", roles::routes())
}
