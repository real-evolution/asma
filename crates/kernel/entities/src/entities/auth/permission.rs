use std::ops::{BitAnd, BitOr, BitXor, Not};

use derive_more::{Display, From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Role;
use crate::traits::*;

#[EnumRepr(type = "i64")]
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize_repr,
    Display,
    Eq,
    Hash,
    PartialEq,
    Serialize_repr,
)]
pub enum Resource {
    Unknown = 0,
    Users = 1,
    Accounts = 2,
    Roles = 3,
    Permissions = 4,
    Channels = 5,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Deserialize, Display, Serialize)]
pub enum Action {
    Global = 1,
    View = 2,
    Add = 4,
    Modify = 8,
    Remove = 16,
}

#[entity(entity_type = "immutable")]
#[derive(Clone, Debug, From, Into)]
pub struct Permission {
    pub resource: Resource,
    pub actions: Actions,
    pub role_id: Key<Role>,
}

#[repr(transparent)]
#[derive(
    Clone, Copy, Debug, Deserialize, Default, From, Into, PartialEq, Serialize,
)]
pub struct Actions(i32);

impl From<Action> for Actions {
    fn from(value: Action) -> Self {
        Self(value as i32)
    }
}

impl Actions {
    pub fn has<A: Into<Self> + Copy>(&self, rhs: &A) -> bool {
        let rhs: Self = (*rhs).into();

        (self.0 & rhs.0) == rhs.0
    }
}

impl BitAnd for Action {
    type Output = Actions;

    fn bitand(self, rhs: Self) -> Self::Output {
        Actions(self as i32 & rhs as i32)
    }
}

impl BitOr for Action {
    type Output = Actions;

    fn bitor(self, rhs: Self) -> Self::Output {
        Actions(self as i32 | rhs as i32)
    }
}

impl BitXor for Action {
    type Output = Actions;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Actions(self as i32 ^ rhs as i32)
    }
}

impl BitAnd for Actions {
    type Output = Actions;

    fn bitand(self, rhs: Actions) -> Self::Output {
        Actions(self.0 | rhs.0)
    }
}

impl BitOr for Actions {
    type Output = Actions;

    fn bitor(self, rhs: Actions) -> Self::Output {
        Actions(self.0 | rhs.0)
    }
}

impl BitXor for Actions {
    type Output = Actions;

    fn bitxor(self, rhs: Actions) -> Self::Output {
        Actions(self.0 ^ rhs.0)
    }
}

impl BitAnd<Action> for Actions {
    type Output = Actions;

    fn bitand(self, rhs: Action) -> Self::Output {
        Actions(self.0 & rhs as i32)
    }
}

impl BitOr<Action> for Actions {
    type Output = Actions;

    fn bitor(self, rhs: Action) -> Self::Output {
        Actions(self.0 | rhs as i32)
    }
}

impl BitXor<Action> for Actions {
    type Output = Actions;

    fn bitxor(self, rhs: Action) -> Self::Output {
        Actions(self.0 ^ rhs as i32)
    }
}

impl Not for Actions {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<i64> for Resource {
    fn from(value: i64) -> Self {
        Self::from_repr(value).unwrap_or(Resource::Unknown)
    }
}

impl Into<i64> for Resource {
    fn into(self) -> i64 {
        self.repr()
    }
}
