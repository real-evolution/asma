use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum AuthError {
    #[display(fmt = "invalid username or password")]
    InvalidUsernameOrPassword,
}
