use crate::entity::Entity;
use domain_macros::{add_entity_fields, Entity};

#[repr(u8)]
pub enum UserState {
    Active = 0,
    Inactive = 1,
    Suspended = 2,
}

#[derive(Entity)]
#[add_entity_fields]
pub struct User {
    pub display_name: String,
    pub state: UserState,
}
