mod dtos;
mod roles;
mod signin;
mod users;

use aide::axum::ApiRouter;
use axum::routing::post;
use driver_web_common::state::AppState;

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .route("/signin", post(signin::signin))
        .route("/refresh", post(signin::refresh))
        .nest("/roles", roles::routes())
        .nest("/users", users::routes())
}
