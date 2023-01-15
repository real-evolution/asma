use axum::{extract::FromRequestParts, http::request::Parts};
use driver_web_common::{
    auth::config::{AuthTokenConfig, AUTH_TOKEN_CONFIG_SECTION},
    state::AppState,
};
use kernel_services::config::ConfigService;

use crate::{
    error::{ApiError, ApiResult},
    util::auth::config::RestAuthTokenConfig,
};

#[async_trait::async_trait]
impl FromRequestParts<AppState> for RestAuthTokenConfig {
    type Rejection = ApiError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> ApiResult<Self> {
        let config = state
            .config
            .get_section::<AuthTokenConfig>(AUTH_TOKEN_CONFIG_SECTION)?;

        Ok(config.into())
    }
}
