use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::Extension;
use axum::TypedHeader;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, Validation};

use crate::config::ApiConfig;
use crate::error::{ApiError, ApiResult};
use crate::util::claims::Claims;

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> ApiResult<Self> {
        let auth = TypedHeader::<Authorization<Bearer>>::from_request_parts(
            parts, state,
        )
        .await
        .map_err(|err| {
            warn!("client sent a bad token: {err:?}");
            Error::from(ErrorKind::InvalidToken)
        })?;

        let token_conf =
            Extension::<ApiConfig>::from_request_parts(parts, state)
                .await
                .expect("could not read api config using `axum::Extension<T>`");

        Ok(jsonwebtoken::decode::<Claims>(
            &auth.token(),
            &DecodingKey::from_secret(token_conf.token.signing_key.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?)
    }
}
