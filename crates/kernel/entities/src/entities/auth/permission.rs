use std::ops::{BitAnd, BitOr, BitXor, Not};

use derive_more::{Display, From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;
use schemars::{JsonSchema, JsonSchema_repr};
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
    JsonSchema_repr,
    PartialEq,
    Serialize_repr,
)]
pub enum Resource {
    // auth
    Unknown = 0,
    User = 1,
    Account = 2,
    Role = 3,
    Permission = 4,
    Session = 5,

    // link
    Channel = 6,
    Instance = 7,
    InstanceGroup = 8,

    // comm
    Chat = 9,
    Message = 10,
    Bot = 11,
    Menu = 12,
}

#[EnumRepr(type = "i32")]
#[derive(Clone, Copy, Debug, JsonSchema_repr, Deserialize, Serialize)]
pub enum Action {
    Global = 1,
    View = 2,
    Add = 4,
    Modify = 8,
    Remove = 16,
}

#[entity(entity_type = "immutable")]
#[derive(Clone, Debug, From, Into, JsonSchema)]
pub struct Permission {
    pub resource: Resource,
    pub actions: Actions,
    pub role_id: Key<Role>,
}

#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    From,
    Into,
    PartialEq,
    JsonSchema,
    Deserialize,
    Serialize,
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
        Actions(self.0 & rhs.0)
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

impl From<Resource> for i64 {
    fn from(val: Resource) -> Self {
        val.repr()
    }
}
