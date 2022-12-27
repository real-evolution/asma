use kernel_services::{error::AppError, link::error::LinkError};

pub(super) fn map_ipc_error<E: Into<anyhow::Error>>(err: E) -> AppError {
    LinkError::MessagePassing(err.into()).into()
}

pub(super) fn map_params_error<E: ToString>(err: E) -> AppError {
    LinkError::InvalidParams(err.to_string()).into()
}
