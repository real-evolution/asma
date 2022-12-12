use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
    redoc::Redoc,
};
use axum::{Extension, Router};
use cached::proc_macro::once;
use driver_web_common::state::AppState;
use kernel_services::error::AppResult;

use crate::api;

const OPENAPI_PATH: &str = "/openapi.json";

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
            title: "asma".into(),
            description: Some("Advanced Social Media Aggregator".into()),
            version: "0.0.1-dev".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    Ok(ApiRouter::new()
        .route(
            "/redoc",
            Redoc::new(OPENAPI_PATH)
                .axum_route()
                .with_state::<AppState>(()),
        )
        .nest("/api", api::api_routes()?)
        .route(OPENAPI_PATH, get(serve_api))
        .finish_api(&mut api)
        .layer(Extension(api)))
}
