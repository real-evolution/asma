use super::User;
use crate::{key_type, traits::*};

use chrono::{DateTime, Utc};
use kernel_proc_macros::*;

pub enum AccountState {
    Active,
    Inactive,
}

pub struct AccountPassword {
    pub hash: String,
    pub salt: String,
}

#[entity]
pub struct Account {
    pub holder_name: Option<String>,
    pub password: Option<AccountPassword>,
    pub valid_until: DateTime<Utc>,
    pub state: AccountState,
    pub account_id: key_type!(User),
}
