use kernel_proc_macros::*;

use super::AccountKey;
use crate::traits::*;

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Role {
    pub code: String,
    pub friendly_name: Option<String>,
    pub enabled: bool,
}

#[entity(entity_type = "immutable")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountRole {
    pub account_id: AccountKey,
    pub role_id: RoleKey,
    pub enabled: bool,
}
