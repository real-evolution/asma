use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("account has no password set")]
    UnsetPassword,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("used old password is wrong")]
    OldPasswordWrong,

    #[error("maximum number of seassons ({0}) has been reached")]
    MaxSessionsCountReached(usize),

    #[error("account not withenticated")]
    NotAuthenticated,

    #[error("invalid role: {0}")]
    InvalidRole(String),

    #[error(
        "account `{account_name}` tried to signin with an inactvie user `{username}`"
    )]
    InactiveUser {
        username: String,
        account_name: String,
    },

    #[error(
        "inactive or suspended account `{account_name}` of user `{username}` tried to signi "
    )]
    InactiveAccount {
        username: String,
        account_name: String,
    },
}
