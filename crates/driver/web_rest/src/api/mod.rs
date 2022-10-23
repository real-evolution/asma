pub mod auth;
pub mod diag;

use axum::Router;

pub fn api_routes() -> Router {
    debug!("creating router with Swagger/OpenAPI support");

    Router::new()
        .nest("/diag", diag::routes())
        .nest("/auth", auth::routes())
}
