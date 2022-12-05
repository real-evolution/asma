use std::net::SocketAddr;

use axum::{extract::*, routing::post, Router};
use chrono::Utc;
use driver_web_common::state::AppState;

use crate::{error::ApiResult, extractors::validated_json::ValidatedJson};

pub async fn setup(
    State(state): State<AppState>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    ValidatedJson(form): ValidatedJson<dtos::RootAccountDetails>,
) -> ApiResult<()> {
    info!("a setup request was made from: `{ip}`");

    state.setup.setup(form.holder_name, form.password).await?;

    info!("system was setup successfully at: {}", Utc::now());
    Ok(())
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(setup))
}

pub mod dtos {
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Clone, Debug, Deserialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct RootAccountDetails {
        #[validate(length(min = 4))]
        pub holder_name: Option<String>,
        #[validate(length(min = 8))]
        pub password: String,
    }
}
