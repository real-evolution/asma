use axum::{extract::FromRequestParts, http::request::Parts};
use driver_web_common::state::AppState;
use kernel_services::config::ConfigService;

use crate::{
    config::{ApiConfig, API_CONFIG_SECTION},
    error::{ApiError, ApiResult},
};

#[async_trait::async_trait]
impl FromRequestParts<AppState> for ApiConfig {
    type Rejection = ApiError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> ApiResult<Self> {
        Ok(state.config.get_section::<ApiConfig>(API_CONFIG_SECTION)?)
    }
}
