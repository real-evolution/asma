use std::fmt::Debug;

use kernel_repositories::error::RepoError;
use kernel_services::{
    error::{AppError, AuthError},
    setup::error::SetupError,
};
use tonic::Status;

pub(crate) trait IntoStatus {
    fn into_status(self) -> Status;
}

pub(crate) trait IntoStatusResult<T> {
    fn into_status_result(self) -> Result<T, Status>;
}

impl IntoStatus for RepoError {
    fn into_status(self) -> Status {
        match self {
            | RepoError::Io(_) | RepoError::Data(_) => {
                Status::internal("internal error")
            }
            | RepoError::NotFound => Status::not_found("item not found"),
            | RepoError::AlreadyExists => {
                Status::already_exists("item already exists")
            }
            | RepoError::DuplicateValue(_) => {
                Status::already_exists("duplicate value")
            }
            | RepoError::InvalidParameter(_)
            | RepoError::Serialization(_)
            | RepoError::Deserialization(_) => {
                Status::invalid_argument("invalid argument")
            }
        }
    }
}

impl IntoStatus for AuthError {
    fn into_status(self) -> Status {
        match self {
            | AuthError::UnsetPassword => Status::failed_precondition(
                "account does not have a password yet",
            ),
            | AuthError::InvalidCredentials => {
                Status::unauthenticated("invalid credentials")
            }
            | AuthError::OldPasswordWrong => {
                Status::permission_denied("invalid old password")
            }
            | AuthError::MaxSessionsCountReached(n) => {
                Status::resource_exhausted(format!(
                    "maximum sessions count of {n} reached"
                ))
            }
            | AuthError::NotAuthenticated => {
                Status::unauthenticated("not authenticated")
            }
            | AuthError::InvalidRole(_) => {
                Status::permission_denied("invalid roles")
            }
            | AuthError::InactiveUser {
                username: _,
                account_name: _,
            } => Status::failed_precondition("inactive user"),
            | AuthError::InactiveAccount {
                username: _,
                account_name: _,
            } => Status::failed_precondition("inactive account"),
        }
    }
}

impl IntoStatus for SetupError {
    fn into_status(self) -> Status {
        match self {
            | SetupError::AlreadySetup => {
                Status::unavailable("the system has already been set up")
            }
        }
    }
}

impl IntoStatus for AppError {
    fn into_status(self) -> Status {
        match self {
            | AppError::Unknown(_) => Status::unknown("unknown error"),
            | AppError::Setup(err) => err.into_status(),
            | AppError::Auth(err) => err.into_status(),
            | AppError::Repo(err) => err.into_status(),
            | _ => Status::internal("internal error"),
        }
    }
}

impl<T, E> IntoStatusResult<T> for Result<T, E>
where
    E: Debug + IntoStatus,
{
    fn into_status_result(self) -> Result<T, Status> {
        match self {
            | Ok(value) => Ok(value),
            | Err(err) => {
                error!("an error occured during gRPC call: {err:#?}");

                Err(err.into_status())
            }
        }
    }
}
