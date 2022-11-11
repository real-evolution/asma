use std::ops::{BitAnd, BitOr, BitXor, Not};

use derive_more::Display;
use kernel_proc_macros::*;

use super::UserKey;
use crate::traits::*;

#[repr(i64)]
#[derive(Clone, Debug, Display, sqlx::Type)]
pub enum Resource {
    ThisUser,
    ThisAccount,
    Users,
    Accounts,
    Roles,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Display, sqlx::Type)]
pub enum Action {
    View,
    Add,
    Modify,
    Remove,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Permission {
    pub resource: Resource,
    pub actions: Actions,
    pub role_id: UserKey,
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Actions(i32);

impl Actions {
    pub fn inner(self) -> i32 {
        self.0 as i32
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
