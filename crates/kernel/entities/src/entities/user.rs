use crate::traits::*;
use kernel_proc_macros::*;

#[derive(sqlx::Type)]
#[repr(i32)]
pub enum UserState {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
}

#[entity]
#[derive(sqlx::FromRow)]
pub struct User {
    pub display_name: String,
    pub username: String,
    pub state: UserState,
}
