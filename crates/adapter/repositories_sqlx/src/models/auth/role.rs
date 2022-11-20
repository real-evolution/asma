use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities;
use kernel_entities::entities::auth::{AccountKey, RoleKey};
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "roles", id = id, insertable, deletable)]
pub struct Role {
    pub id: Uuid,
    #[ormx(get_one, get_optional = by_code_optional)]
    pub code: String,
    #[ormx(set)]
    pub friendly_name: Option<String>,
    pub is_active: bool,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "account_roles", id = id, insertable, deletable)]
pub struct AccountRole {
    pub id: Uuid,
    #[ormx(custom_type)]
    pub account_id: AccountKey,
    #[ormx(custom_type)]
    pub role_id: RoleKey,
    pub is_active: bool,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(entities::auth::Role, Role, 6);
generate_mapping!(entities::auth::AccountRole, AccountRole, 6);
