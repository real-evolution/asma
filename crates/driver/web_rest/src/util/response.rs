use std::fmt::Display;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

pub struct Created<K, P = String>(pub P, pub K);

impl<K: Display, P: Display> IntoResponse for Created<K, P> {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("{}/{}", self.0, self.1))],
        )
            .into_response()
    }
}

impl<'a, K> Into<Created<K, String>> for Created<K, &'a str> {
    fn into(self) -> Created<K, String> {
        Created(self.0.to_string(), self.1)
    }
}
