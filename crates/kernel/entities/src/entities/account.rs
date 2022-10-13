use crate::traits::*;
use domain_proc_macros::*;

use chrono::{DateTime, Utc};

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
}
