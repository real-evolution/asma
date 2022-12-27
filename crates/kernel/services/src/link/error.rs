use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinkError {
    #[error("internal error: {0}")]
    InternalError(anyhow::Error),

    #[error("unsupported event: {0}")]
    UnsupportedEvent(String),

    #[error("communication error: {0}")]
    Communication(String),

    #[error("invalid params: {0}")]
    InvalidParams(String),

    #[error("invalid channel state: {0}")]
    InvalidChannelState(String),

    #[error("ipc error: {0}")]
    MessagePassing(anyhow::Error),
}
