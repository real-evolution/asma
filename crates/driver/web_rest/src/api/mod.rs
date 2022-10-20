mod diag;

use axum::Router;

pub fn api_routes() -> Router {
    Router::new().nest("/api/diag", diag::diag_routes())
}
