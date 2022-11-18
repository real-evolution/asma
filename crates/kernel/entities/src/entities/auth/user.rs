use kernel_proc_macros::*;

use crate::traits::*;

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub display_name: String,
    pub username: String,
    pub is_active: bool,
}
