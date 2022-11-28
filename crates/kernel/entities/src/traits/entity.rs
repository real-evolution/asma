use std::{fmt::Display, marker::PhantomData};

use chrono::{DateTime, Utc};

pub type KeyType = uuid::Uuid;

pub trait Entity: Sized {
    fn id(&self) -> &Key<Self>;
    fn created_at(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: Entity {
    fn updated_at(&self) -> DateTime<Utc>;
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
pub struct Key<E, T = KeyType>(T, #[serde(skip)] PhantomData<E>);

impl<E, T: Clone> Key<E, T> {
    pub fn new(inner: T) -> Self {
        Self(inner, Default::default())
    }

    pub fn value_ref(&self) -> &T {
        &self.0
    }

    pub fn value(&self) -> T {
        self.0.clone()
    }
}

impl<E, T: Clone> From<T> for Key<E, T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<E> Into<KeyType> for Key<E> {
    fn into(self) -> KeyType {
        self.value()
    }
}

impl<E, T: Display> Display for Key<E, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
