use kernel_proc_macros::*;

use super::*;
use crate::traits::*;

#[repr(i32)]
#[derive(Debug, Clone, sqlx::Type)]
pub enum AccountState {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Account {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
    pub user_id: UserKey,
}
