use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use validator::Validate;

use crate::api::dtos::pagination::Pagination;
use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> ApiResult<Self> {
        let Query(pagination) =
            Query::<Pagination>::from_request_parts(parts, state).await?;

        pagination.validate()?;

        Ok(pagination)
    }
}
