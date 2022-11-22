use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities;
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "roles", id = id, insertable, deletable)]
pub struct RoleModel {
    #[ormx(default)]
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
pub struct AccountRoleModel {
    #[ormx(default)]
    pub id: Uuid,
    pub account_id: Uuid,
    pub role_id: Uuid,
    pub is_active: bool,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

#[derive(ormx::Patch)]
#[ormx(table_name = "roles", table = RoleModel, id = "id")]
pub struct UpdateSessionModel {
    pub friendly_name: Option<String>,
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(entities::auth::Role, RoleModel, 6);
generate_mapping!(entities::auth::AccountRole, AccountRoleModel, 6);
