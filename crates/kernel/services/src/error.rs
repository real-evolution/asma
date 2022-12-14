use thiserror::Error;

pub use crate::auth::error::AuthError;
pub use crate::config::error::ConfigError;
pub use crate::crypto::error::CryptoError;
use crate::link::error::LinkError;
use crate::setup::error::SetupError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),

    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    #[error("crypto error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("setup error: {0}")]
    Setup(#[from] SetupError),

    #[error("auth error: {0}")]
    Auth(#[from] AuthError),

    #[error("link error: {0}")]
    Link(#[from] LinkError),

    #[error("repo error: {0}")]
    Repo(#[from] kernel_repositories::error::RepoError),
}
