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
    Root = 0,
    Admin = 1,
    Regular = 2,
}

#[entity]
#[derive(sqlx::FromRow)]
pub struct User {
    pub display_name: String,
    pub username: String,
    pub state: UserState,
    pub level: UserLevel,
}
