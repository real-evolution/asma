use axum::body::HttpBody;
use axum::extract::{FromRequest, Json as ExtractJson, RequestParts};
use axum::BoxError;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait::async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ApiError;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let ExtractJson(value) = ExtractJson::<T>::from_request(req).await?;

        value.validate()?;

        Ok(ValidatedJson(value))
    }
}
