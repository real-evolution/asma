use aide::OperationIo;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, Copy, Default, OperationIo)]
#[aide(input)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> ApiResult<Self> {
        let Query(query) = Query::<T>::from_request_parts(parts, state).await?;

        query.validate()?;

        Ok(Self(query))
    }
}
