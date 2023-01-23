use aide::OperationIo;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use derive_more::{Deref, From, Into};
use driver_web_common::value_types::Pagination;
use validator::Validate;

use crate::error::{ApiError, ApiResult};

#[derive(Clone, Debug, Deref, Into, From, OperationIo)]
#[aide(input)]
pub struct QueryPagination(Pagination);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for QueryPagination
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

        Ok(Self(pagination))
    }
}
