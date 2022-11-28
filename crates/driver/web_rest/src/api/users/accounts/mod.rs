pub mod view;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(view::get_accounts_of))
}
