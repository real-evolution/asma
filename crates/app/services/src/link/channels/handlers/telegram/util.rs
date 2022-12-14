use kernel_services::{error::AppError, link::error::LinkError};

pub(super) fn map_request_error(err: teloxide::RequestError) -> AppError {
    LinkError::InternalError(err.into()).into()
}
