use axum::extract::{FromRequest, RequestParts};
use axum::{body::HttpBody, BoxError, Extension};
use axum_auth::AuthBearer;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, Validation};

use crate::config::ApiConfig;
use crate::error::ApiError;
use crate::util::jwt::Claims;

#[async_trait::async_trait]
impl<B> FromRequest<B> for Claims
where
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ApiError;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let bearer = AuthBearer::from_request(req).await.map_err(|err| {
            warn!("client sent a bad token: {err:?}");
            Error::from(ErrorKind::InvalidToken)
        })?;

        let token_conf = Extension::<ApiConfig>::from_request(req)
            .await
            .expect("could not read api config using `axum::Extension<T>`");

        Ok(jsonwebtoken::decode::<Claims>(
            &bearer.0,
            &DecodingKey::from_secret(token_conf.token.signing_key.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?)
    }
}
