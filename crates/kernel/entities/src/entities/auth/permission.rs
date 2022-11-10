use derive_more::Display;
use enumflags2::{bitflags, BitFlags};
use kernel_proc_macros::*;

use super::UserKey;
use crate::traits::*;

#[repr(u64)]
#[derive(Clone, Debug, Display, sqlx::Type)]
pub enum Resource {
    ThisUser,
    ThisAccount,
    Users,
    Accounts,
    Roles,
}

#[bitflags]
#[repr(u8)]
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
    pub actions: BitFlags<Action>,
    pub role_id: UserKey,
}
