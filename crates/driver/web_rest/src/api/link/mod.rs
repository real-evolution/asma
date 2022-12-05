pub mod channels;

use axum::Router;
use driver_web_common::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/channels", channels::routes())
}
