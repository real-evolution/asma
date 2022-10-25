use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("encoding error: {0}")]
    Encoding(String),

    #[error("memory error: {0}")]
    Memory(String),

    #[error("input was too short (max: {min}, got: {got})")]
    InputTooShort { min: usize, got: usize },

    #[error("input was too long (max: {max}, got: {got})")]
    InputTooLong { max: usize, got: usize },

    #[error("version error: {0}")]
    Version(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("format error: {0}")]
    Format(String),

    #[error("hash error: {0}")]
    Hash(String),

    #[error("salt error: {0}")]
    Salt(String),

    #[error("unsupported: {0}")]
    Unsupported(String),
}
