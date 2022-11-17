use std::fmt::Display;

use kernel_proc_macros::*;

use super::AccountKey;
use crate::traits::*;

const ROOT_ROLE_CODE: &str = "root";
const ADMIN_ROLE_CODE: &str = "admin";
const REGULAR_ROLE_CODE: &str = "regular";

#[derive(Clone, Debug)]
pub enum KnownRoles {
    Root,
    Admin,
    Regular,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Role {
    pub code: String,
    pub friendly_name: Option<String>,
    pub is_active: bool,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountRole {
    pub account_id: AccountKey,
    pub role_id: RoleKey,
    pub is_active: bool,
}

impl Into<&str> for KnownRoles {
    fn into(self) -> &'static str {
        match self {
            KnownRoles::Root => ROOT_ROLE_CODE,
            KnownRoles::Admin => ADMIN_ROLE_CODE,
            KnownRoles::Regular => REGULAR_ROLE_CODE,
        }
    }
}

impl Display for KnownRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.clone().into())
    }
}
