use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use kernel_repositories::error::RepoError;
use kernel_services::error::AppError;
use serde_json::json;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("internal error occured")]
    Internal(#[from] anyhow::Error),

    #[error(transparent)]
    Json(#[from] JsonRejection),

    #[error(transparent)]
    Query(#[from] QueryRejection),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("authorization error: {0}")]
    Authorization(String),

    #[error(transparent)]
    App(#[from] AppError),
}

fn status_tuple(status: StatusCode) -> (StatusCode, String) {
    (status, status.to_string())
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::Internal(err) => {
                error!("internal error: {err:?}");
                status_tuple(StatusCode::INTERNAL_SERVER_ERROR)
            }

            ApiError::Serialization(err) => {
                error!("serialization error: {err:?}");
                status_tuple(StatusCode::INTERNAL_SERVER_ERROR)
            }

            ApiError::Json(_) | ApiError::Query(_) => {
                (StatusCode::BAD_REQUEST, "invalid request data".into())
            }

            ApiError::Validation(_) => {
                (StatusCode::BAD_REQUEST, "invalid request data".into())
            }

            ApiError::Jwt(err) => (StatusCode::UNAUTHORIZED, err.to_string()),

            ApiError::Authorization(err) => {
                (StatusCode::FORBIDDEN, err.to_owned())
            }

            ApiError::App(err) => match err {
                AppError::Repo(err) => match err {
                    RepoError::NotFound => {
                        (StatusCode::NOT_FOUND, err.to_string())
                    }
                    _ => status_tuple(StatusCode::INTERNAL_SERVER_ERROR),
                },

                AppError::Auth(err) => {
                    (StatusCode::UNAUTHORIZED, err.to_string())
                }

                _ => status_tuple(StatusCode::INTERNAL_SERVER_ERROR),
            },
        };

        warn!("{status}: {self:?}");

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<RepoError> for ApiError {
    fn from(value: RepoError) -> Self {
        AppError::from(value).into()
    }
}
