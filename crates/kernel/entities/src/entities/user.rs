use kernel_proc_macros::*;

use crate::traits::*;

#[derive(sqlx::Type)]
#[repr(i32)]
pub enum UserState {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
}

#[derive(sqlx::Type)]
#[repr(i32)]
pub enum UserLevel {
    Root = 2,
    Admin = 1,
    Normal = 0,
}

#[entity]
#[derive(sqlx::FromRow)]
pub struct User {
    pub display_name: String,
    pub username: String,
    pub level: UserLevel,
    pub state: UserState,
}
