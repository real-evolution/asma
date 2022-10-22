mod auth;
mod diag;

use axum::Router;

pub fn api_routes() -> Router {
    Router::new()
        .nest("/diag", diag::routes())
        .nest("/auth", auth::routes())
}
