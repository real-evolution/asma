use std::ops::{BitAnd, BitOr, BitXor, Not};

use derive_more::Display;
use enum_repr::EnumRepr;
use kernel_proc_macros::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::UserKey;
use crate::traits::*;

lazy_static! {
    static ref ALL_ACTIONS: Vec<Action> = vec![
        Action::Global,
        Action::View,
        Action::Add,
        Action::Modify,
        Action::Remove,
    ];
}

#[EnumRepr(type = "i64")]
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize_repr,
    Display,
    Eq,
    PartialEq,
    Serialize_repr,
    sqlx::Type,
)]
pub enum Resource {
    Users = 0,
    Accounts = 1,
    Roles = 2,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Deserialize, Display, Serialize, sqlx::Type)]
pub enum Action {
    Global = 1,
    View = 2,
    Add = 4,
    Modify = 8,
    Remove = 16,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Permission {
    pub resource: Resource,
    pub actions: Actions,
    pub role_id: UserKey,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Actions(i32);

impl Resource {
    pub fn all() -> Vec<Self> {
        vec![Resource::Users, Resource::Accounts, Resource::Roles]
    }
}

impl Actions {
    pub fn from_bits(inner: i32) -> Self {
        Self(inner)
    }

    pub fn all() -> Self {
        Self::all_iter().fold(Self(0), |a, b| a | b)
    }

    pub fn all_iter() -> impl Iterator<Item = Action> {
        ALL_ACTIONS.clone().into_iter()
    }

    pub fn inner(self) -> i32 {
        self.0 as i32
    }

    pub fn has(&self, rhs: Self) -> bool {
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
