use aide::OperationIo;
use axum::{
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    BoxError,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, Copy, Default, OperationIo)]
#[aide(input)]
pub struct ValidatedJson<T>(pub T);

#[async_trait::async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    B: Send + HttpBody + 'static,
    B::Error: Send + Sync + Into<BoxError>,
    B::Data: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request<B>, state: &S) -> ApiResult<Self> {
        let Json(object) = Json::<T>::from_request(req, state).await?;

        object.validate()?;

        Ok(Self(object))
    }
}
