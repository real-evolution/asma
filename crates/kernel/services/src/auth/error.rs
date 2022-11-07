use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("account has no password set")]
    UnsetPassword,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("maximum number of seassons ({0}) has been reached")]
    MaxSessionsCountReached(usize),

    #[error("account not withenticated")]
    NotAuthenticated,

    #[error("invalid role: {0}")]
    InvalidRole(String),
}
