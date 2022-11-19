use std::fmt::Display;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

pub struct Created<K>(pub &'static str, pub K);

impl<K: Display> IntoResponse for Created<K> {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("{}/{}", self.0, self.1))],
        )
            .into_response()
    }
}
