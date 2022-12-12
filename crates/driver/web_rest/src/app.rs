use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
};
use axum::{Extension, Router};
use cached::proc_macro::once;
use driver_web_common::state::AppState;
use kernel_services::error::AppResult;

use crate::api;

#[once]
fn serialize_api(api: &OpenApi) -> String {
    serde_json::to_string(api).expect("could not serialize api")
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    serialize_api(&api)
}

pub fn make_app() -> AppResult<Router<AppState>> {
    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    Ok(ApiRouter::new()
        .route("/api.json", get(serve_api))
        .nest("/api", api::api_routes()?)
        .finish_api(&mut api)
        .layer(Extension(api)))
}
