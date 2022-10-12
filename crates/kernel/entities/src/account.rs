use crate::entity::Entity;
use chrono::{DateTime, Utc};
use domain_macros::{add_entity_fields, Entity};

pub enum AccountState {
    Active,
    Inactive,
}

pub struct AccountPassword {
    pub hash: String,
    pub salt: String,
}

#[derive(Entity)]
#[add_entity_fields]
pub struct Account {
    pub holder_name: Option<String>,
    pub password: Option<AccountPassword>,
    pub valid_until: DateTime<Utc>,
    pub state: AccountState,
}
