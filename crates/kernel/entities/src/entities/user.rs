use crate::traits::*;
use kernel_proc_macros::*;

pub enum UserState {
    Active,
    Inactive,
    Suspended,
}

#[entity]
pub struct User {
    pub display_name: String,
    pub state: UserState,
}
