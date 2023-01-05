use kernel_services::{error::{AppError, AppResult}, link::error::LinkError};
use serde::de::DeserializeOwned;

pub(super) fn deserialize<T: DeserializeOwned>(buf: &[u8]) -> AppResult<T> {
    rmp_serde::from_slice(buf).map_err(map_params_error)
}

pub(super) fn map_ipc_error<E: Into<anyhow::Error>>(err: E) -> AppError {
    LinkError::MessagePassing(err.into()).into()
}

pub(super) fn map_params_error<E: ToString>(err: E) -> AppError {
    LinkError::InvalidParams(err.to_string()).into()
}
