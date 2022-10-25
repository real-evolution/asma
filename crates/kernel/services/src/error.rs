use kernel_repositories::error::RepoError;
use thiserror::Error;

pub use crate::crypto::error::CryptoError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config key `{0}` not found")]
    NotFound(String),

    #[error("failed to parse path `{0}`")]
    PathParse(String),

    #[error("failed to parse file `{uri}`: {error}")]
    FileParse { uri: String, error: String },

    #[error("failed to parse value: {0}")]
    ValueParse(String),

    #[error("deserialization error: {0}")]
    Deserialization(#[from] erased_serde::Error),

    #[error("unknown error: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("maximum number of seassons ({0}) has been reached")]
    MaxSessionsCountReached(u32),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),

    #[error("config error: {0}")]
    Config(ConfigError),

    #[error("crypto error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("repo error: {0}")]
    Repo(#[from] RepoError),

    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
}

impl From<erased_serde::Error> for AppError {
    fn from(err: erased_serde::Error) -> Self {
        AppError::Config(err.into())
    }
}
