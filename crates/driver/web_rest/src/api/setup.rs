use axum::{routing::post, Json, Router};
use axum_client_ip::ClientIp;
use chrono::Utc;
use kernel_services::setup::SetupService;

use crate::{error::ApiResult, extractors::di::Dep};
use crate::{error::ApiResult, extractors::validated_json::ValidatedJson};

pub async fn setup(
    setup_svc: Dep<dyn SetupService>,
    ClientIp(ip): ClientIp,
    Json(form): Json<dtos::RootAccountDetails>,
) -> ApiResult<()> {
    info!("a setup request was made from: `{ip}`");

    setup_svc.setup(form.holder_name, form.password).await?;

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
