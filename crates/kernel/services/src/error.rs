use crate::auth::AuthError;

use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum ServiceError {
    #[display(fmt = "auth error: {}", _0)]
    Auth(AuthError),
}
