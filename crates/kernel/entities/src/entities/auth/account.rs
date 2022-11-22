use derive_more::{From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;

use super::*;
use crate::traits::*;

#[EnumRepr(type = "i32")]
#[derive(
    Clone, Copy, Debug, serde::Deserialize, serde::Serialize, sqlx::Type,
)]
pub enum AccountState {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
}

#[entity]
#[derive(Clone, Debug, From, Into, sqlx::FromRow)]
pub struct Account {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
    pub user_id: Key<User>,
}
