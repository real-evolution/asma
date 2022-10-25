use axum::{
    extract::rejection::JsonRejection, http::StatusCode,
    response::IntoResponse, Json,
};
use kernel_repositories::error::RepoError;
use kernel_services::error::AppError;
use serde_json::json;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("internal error occured")]
    InternalError(#[from] anyhow::Error),

    #[error(transparent)]
    JsonError(#[from] JsonRejection),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AppError(#[from] AppError),
}

fn status_tuple(status: StatusCode) -> (StatusCode, String) {
    (status, status.to_string())
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::InternalError(err) => {
                error!("internal error: {err:?}");
                status_tuple(StatusCode::INTERNAL_SERVER_ERROR)
            }

            ApiError::JsonError(_) => {
                (StatusCode::BAD_REQUEST, "invalid request schema".into())
            }

            ApiError::ValidationError(_) => {
                (StatusCode::BAD_REQUEST, "invalid request data".into())
            }

            ApiError::AppError(err) => match err {
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
