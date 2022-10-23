use axum::{
    extract::rejection::JsonRejection, http::StatusCode,
    response::IntoResponse, Json,
};
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

    #[error("invalid credentials")]
    InvalidCredentials,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::InternalError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server occured")
            }

            ApiError::JsonError(_) => {
                (StatusCode::BAD_REQUEST, "invalid request schema")
            }

            ApiError::ValidationError(_) => {
                (StatusCode::BAD_REQUEST, "invalid request data")
            }

            ApiError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "invalid credentails were used")
            }
        };

        warn!("{status}: {self}");

        (status, Json(json!({ "error": message }))).into_response()
    }
}
