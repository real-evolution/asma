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
pub struct Key<E>(
    #[serde(
        serialize_with = "bson::serde_helpers::uuid_1_as_binary::serialize",
        deserialize_with = "bson::serde_helpers::uuid_1_as_binary::deserialize"
    )]
    KeyType,
    #[serde(skip)] PhantomData<E>,
);

impl<E> Key<E> {
    pub fn new(inner: uuid::Uuid) -> Self {
        Self(inner, Default::default())
    }

    pub fn value_ref(&self) -> &uuid::Uuid {
        &self.0
    }

    pub fn value(&self) -> uuid::Uuid {
        self.0
    }
}

impl<E> From<KeyType> for Key<E> {
    fn from(value: KeyType) -> Self {
        Self(value, Default::default())
    }
}

impl<E> FromStr for Key<E> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = uuid::Uuid::from_str(s)?;

        Ok(Self(inner, PhantomData))
    }
}

impl<E> From<Key<E>> for KeyType {
    fn from(val: Key<E>) -> Self {
        val.value()
    }
}

impl<E> Display for Key<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E> std::hash::Hash for Key<E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<E> PartialEq for Key<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<E> Eq for Key<E> {}
