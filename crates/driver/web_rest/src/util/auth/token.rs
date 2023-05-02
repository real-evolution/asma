use aide::OperationIo;
use derive_more::{Deref, From, Into};
use driver_web_common::auth::{token::AuthToken, validator::FallbackValidator};

use crate::error::ApiError;

#[derive(Debug, Clone, Deref, From, Into, OperationIo)]
#[aide(input)]
#[repr(transparent)]
pub struct RestAuthToken<const ACCEPT_EXPIRED: bool = false>(AuthToken);

impl<const ACCEPT_EXPIRED: bool> FallbackValidator
    for RestAuthToken<ACCEPT_EXPIRED>
{
    type Error = ApiError;

    fn unauthorized(&self) -> Result<&Self, Self::Error> {
        Err(ApiError::Authorization(
            "insufficient permissions".to_owned(),
        ))
    }
}
