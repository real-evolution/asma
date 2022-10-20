use axum::{extract::Path, routing::get, Router};

async fn echo(Path(path): Path<String>) -> String {
    path
}

pub fn diag_routes() -> Router {
    Router::new().route("/echo", get(echo))
}
