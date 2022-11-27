use std::fmt::Display;

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use kernel_entities::traits::{Entity, Key};
use serde::Serialize;

pub struct Created<K, D, P = String>(pub P, pub K, pub D);

impl<K: Display, D: Serialize, P: Display> IntoResponse for Created<K, D, P> {
    fn into_response(self) -> axum::response::Response {
        let Ok(json) = serde_json::to_string(&self.2) else {
            warn!("could not serialize created response");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("{}/{}", self.0, self.1))],
            json,
        )
            .into_response()
    }
}

impl<'a, K, E: Serialize> Into<Created<K, E, String>>
    for Created<K, E, &'a str>
{
    fn into(self) -> Created<K, E, String> {
        Created(self.0.to_string(), self.1, self.2)
    }
}

pub type EntityCreated<E, P = String> = Created<Key<E>, E, P>;

impl<E: Entity + Clone, P> EntityCreated<E, P> {
    pub fn new(path: P, entity: E) -> Self {
        Self(path, entity.id().clone(), entity)
    }
}
