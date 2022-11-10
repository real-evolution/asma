use kernel_proc_macros::*;

use crate::traits::*;

#[repr(i32)]
#[derive(Debug, Clone, sqlx::Type)]
pub enum UserLevel {
    Root = 0,
    Admin = 1,
    Regular = 2,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub display_name: String,
    pub username: String,
    pub is_active: bool,
    pub level: UserLevel,
}
