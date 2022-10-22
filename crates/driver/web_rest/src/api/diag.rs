use axum::{routing::get, Router};

#[utoipa::path(
    get, post,
    path = "/api/diag/echo",
    request_body(content = String, description = "Content to echo"),
    responses((status = 200, description = "Request body", body = [String]))
)]
pub(super) async fn echo(body: String) -> String {
    body
}

pub(super) fn routes() -> Router {
    Router::new().route("/echo", get(echo).post(echo))
}
