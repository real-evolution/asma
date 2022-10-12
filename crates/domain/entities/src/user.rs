use crate::entity::Entity;
use domain_macros::{Entity, add_entity_fields};

#[derive(Entity)]
#[add_entity_fields]
pub struct User {
}
