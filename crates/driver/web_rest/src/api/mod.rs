pub(super) mod auth;
pub(super) mod diag;

use axum::Router;

pub(super) fn api_routes() -> Router {
    debug!("creating router with Swagger/OpenAPI support");

    Router::new()
        .nest("/diag", diag::routes())
        .nest("/auth", auth::routes())
}
