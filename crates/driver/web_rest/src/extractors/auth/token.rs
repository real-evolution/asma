use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use driver_web_common::{auth::token::AuthToken, state::AppState};

use crate::{
    error::{ApiError, ApiResult},
    util::auth::{config::RestAuthTokenConfig, token::RestAuthToken},
};

#[async_trait::async_trait]
impl<const ACCEPT_EXPIRED: bool> FromRequestParts<AppState>
    for RestAuthToken<ACCEPT_EXPIRED>
{
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

        let config =
            RestAuthTokenConfig::from_request_parts(parts, state).await?;

        match AuthToken::decode::<ACCEPT_EXPIRED, _>(
            auth.token(),
            config.into(),
        ) {
            | Ok(token) => Ok(token.into()),
            | Err(err) => {
                warn!("token decoding failed: {err:?}");

                Err(ApiError::Authorization("invalid token".into()))
            }
        }
    }
}
