use crate::entity::Entity;
use domain_macros::{add_entity_fields, Entity};

pub enum UserState {
    Active,
    Inactive,
    Suspended,
}

#[derive(Entity)]
#[add_entity_fields]
pub struct User {
    pub display_name: String,
    pub state: UserState,
}
