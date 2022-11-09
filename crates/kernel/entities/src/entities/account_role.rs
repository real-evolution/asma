use super::*;
use crate::traits::*;

use kernel_proc_macros::*;

#[entity(entity_type = "immutable")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountRole {
    pub account_id: AccountKey,
    pub role_id: RoleKey,
    pub enabled: bool,
}
