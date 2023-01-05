use derive_more::{From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};

use super::*;
use crate::traits::*;

#[EnumRepr(type = "i32")]
#[derive(Clone, Copy, Debug, JsonSchema_repr, Deserialize, Serialize)]
pub enum AccountState {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
}

#[entity]
#[derive(Clone, Debug, From, Into, JsonSchema)]
pub struct Account {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
    pub user_id: Key<User>,
}

impl From<i32> for AccountState {
    fn from(value: i32) -> Self {
        Self::from_repr(value).unwrap_or(AccountState::Inactive)
    }
}

impl From<AccountState> for i32 {
    fn from(val: AccountState) -> Self {
        val.repr()
    }
}

impl From<bool> for AccountState {
    fn from(value: bool) -> Self {
        if value {
            Self::Active
        } else {
            Self::Inactive
        }
    }
}
