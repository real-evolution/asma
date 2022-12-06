use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::TypedHeader;
use driver_web_common::state::AppState;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, Validation};

use crate::config::ApiConfig;
use crate::error::{ApiError, ApiResult};
use crate::util::claims::Claims;

#[async_trait::async_trait]
impl FromRequestParts<AppState> for Claims {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> ApiResult<Self> {
        let auth = TypedHeader::<Authorization<Bearer>>::from_request_parts(
            parts, state,
        )
        .await
        .map_err(|err| {
            warn!("client sent a bad token: {err:?}");
            Error::from(ErrorKind::InvalidToken)
        })?;

        let config = ApiConfig::from_request_parts(parts, state).await?;

        Ok(jsonwebtoken::decode::<Claims>(
            &auth.token(),
            &DecodingKey::from_secret(config.token.signing_key.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?)
    }
}
