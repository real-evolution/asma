pub mod channels;

use axum::Router;

pub fn routes() -> Router {
    Router::new().nest("/channels", channels::routes())
}
