use chrono::{DateTime, Utc};
use common_validation::identifier;
use kernel_entities::{entities::auth::*, traits::Key};
use mapper::Mapper;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Mapper, Serialize)]
#[serde(rename_all = "camelCase")]
#[from(Role)]
pub struct RoleDto {
    pub id: Key<Role>,
    pub code: String,
    pub friendly_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Mapper, Serialize)]
#[serde(rename_all = "camelCase")]
#[from(Permission)]
pub struct PermissionDto {
    pub id: Key<Permission>,
    pub resource: Resource,
    pub actions: Actions,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleWithPermissionsDto {
    #[serde(flatten)]
    pub role: RoleDto,
    pub permissions: Vec<PermissionDto>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleDto {
    #[validate(custom = "identifier")]
    pub code: String,
    #[validate(length(min = 4, max = 64))]
    pub friendly_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPermissionDto {
    pub resource: Resource,
    pub actions: Actions,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleDto {
    #[validate(length(min = 4, max = 64))]
    pub friendly_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddAccountToRoleDto {
    pub user_id: Key<User>,
    pub account_id: Key<Account>,
}

pub type RemoveAccountFromRoleDto = AddAccountToRoleDto;
