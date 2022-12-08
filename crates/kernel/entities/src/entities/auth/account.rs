use derive_more::{From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;

use super::*;
use crate::traits::*;

#[EnumRepr(type = "i32")]
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum AccountState {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
}

#[entity]
#[derive(Clone, Debug, From, Into)]
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

impl Into<i32> for AccountState {
    fn into(self) -> i32 {
        self.repr()
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
