use std::fmt::Display;

use derive_more::{From, Into};
use kernel_proc_macros::*;

use super::AccountKey;
use crate::traits::*;

#[entity]
#[derive(Debug, Clone, From, Into, sqlx::FromRow)]
pub struct Role {
    pub code: String,
    pub friendly_name: Option<String>,
    pub is_active: bool,
}

#[entity]
#[derive(Debug, Clone, From, Into, sqlx::FromRow)]
pub struct AccountRole {
    pub account_id: AccountKey,
    pub role_id: RoleKey,
    pub is_active: bool,
}

#[derive(Clone, Debug)]
pub enum KnownRoles {
    Root,
    Admin,
    Regular,
}

impl Into<&str> for KnownRoles {
    fn into(self) -> &'static str {
        match self {
            KnownRoles::Root => "root",
            KnownRoles::Admin => "admin",
            KnownRoles::Regular => "regular",
        }
    }
}

impl Display for KnownRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.clone().into())
    }
}
