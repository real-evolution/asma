use aide::axum::{routing::get, ApiRouter};
use driver_web_common::state::AppState;

pub async fn echo(body: String) -> String {
    body
}

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("/echo", get(echo).post(echo))
}
