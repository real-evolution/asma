use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use driver_web_common::state::AppState;
use jsonwebtoken::{DecodingKey, Validation};

use crate::{
    config::ApiConfig,
    error::{ApiError, ApiResult},
    util::claims::Claims,
};

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

            match err.reason() {
                | TypedHeaderRejectionReason::Missing => {
                    warn!("client did not send a token: {err:?}");
                    ApiError::Authorization("missing token".into())
                }
                | TypedHeaderRejectionReason::Error(inner) => {
                    warn!("client sent an invalid token: {err:?}, {inner:?}");
                    ApiError::Authorization("invalid token".into())
                }

                | _ => {
                    warn!("unknown token error occured: {err:?}");
                    ApiError::Internal(err.into())
                }
            }
        })?;

        let config = ApiConfig::from_request_parts(parts, state).await?;

        let mut claims = jsonwebtoken::decode::<Claims>(
            auth.token(),
            &DecodingKey::from_secret(config.token.signing_key.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?;

        claims.config = config;

        Ok(claims)
    }
}
