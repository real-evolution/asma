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
