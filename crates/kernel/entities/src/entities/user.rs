use crate::traits::*;
use domain_proc_macros::*;

pub enum UserState {
    Active,
    Inactive,
    Suspended,
}

#[derive(MutableEntity)]
#[mutable_entity]
pub struct User {
    pub display_name: String,
    pub state: UserState,
}
