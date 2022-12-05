use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use driver_web_common::state::AppState;
use kernel_services::get_config;

use crate::config::{ApiConfig, API_CONFIG_SECTION};
use crate::error::{ApiError, ApiResult};

#[async_trait::async_trait]
impl FromRequestParts<AppState> for ApiConfig {
    type Rejection = ApiError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> ApiResult<Self> {
        Ok(get_config!(state.config, API_CONFIG_SECTION => ApiConfig)?)
    }
}
