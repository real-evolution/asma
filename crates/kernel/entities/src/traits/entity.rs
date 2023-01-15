use std::{fmt::Display, marker::PhantomData, str::FromStr};

use chrono::{DateTime, Utc};
use schemars::JsonSchema_repr;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type KeyType = uuid::Uuid;

pub trait Entity: Serialize + DeserializeOwned + Send + Sync + Sized {
    fn id(&self) -> &Key<Self>;
    fn created_at(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: Entity {
    fn updated_at(&self) -> DateTime<Utc>;
}

#[derive(Clone, Copy, Debug, JsonSchema_repr, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
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

impl<E> FromStr for Key<E, KeyType> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = uuid::Uuid::from_str(s)?;

        Ok(Self(inner, PhantomData))
    }
}

impl<E, T: Clone> From<T> for Key<E, T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<E> From<Key<E>> for KeyType {
    fn from(val: Key<E>) -> Self {
        val.value()
    }
}

impl<E, T: Display> Display for Key<E, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E, T: std::hash::Hash> std::hash::Hash for Key<E, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<E, T: PartialEq> PartialEq for Key<E, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<E, T: Eq> Eq for Key<E, T> {}
