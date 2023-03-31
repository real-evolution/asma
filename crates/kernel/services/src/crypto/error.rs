use std::cmp::Ordering;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("encoding error: {0}")]
    Encoding(String),

    #[error("memory error: {0}")]
    Memory(String),

    #[error("input was too short")]
    InputTooShort,

    #[error("input was too long")]
    InputTooLong,

    #[error("invalid input")]
    InvalidInput,

    #[error("invalid output size: provided {provided:?}, expected {expected}")]
    MismatchingOutputLength { provided: Ordering, expected: usize },

    #[error("format error: {0}")]
    Format(String),

    #[error("hash error: {0}")]
    Hash(String),

    #[error("salt error: {0}")]
    Salt(String),

    #[error("hash verification failure: {0}")]
    Verification(String),

    #[error("unsupported")]
    Unsupported,
}
