use std::fmt::Display;

use derive_more::{From, Into};
use kernel_proc_macros::*;

use super::Account;
use crate::traits::*;

#[entity]
#[derive(Clone, Debug, From, Into)]
pub struct Role {
    pub code: String,
    pub friendly_name: Option<String>,
    pub is_active: bool,
}

#[entity]
#[derive(Clone, Debug, From, Into)]
pub struct AccountRole {
    pub account_id: Key<Account>,
    pub role_id: Key<Role>,
    pub is_active: bool,
}

#[derive(Clone, Debug)]
pub enum KnownRoles {
    Root,
    Admin,
    UserOwner,
}

impl Into<&str> for KnownRoles {
    fn into(self) -> &'static str {
        match self {
            KnownRoles::Root => "root",
            KnownRoles::Admin => "admin",
            KnownRoles::UserOwner => "user_owner",
        }
    }
}

impl Display for KnownRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.clone().into())
    }
}
