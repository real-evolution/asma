use aide::OperationIo;
use chrono::{DateTime, Utc};
use common_validation::identifier;
use kernel_entities::{entities::auth::*, traits::Key};
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Role)]
#[aide(output)]
pub struct RoleDto {
    pub id: Key<Role>,
    pub code: String,
    pub friendly_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Permission)]
#[aide(output)]
pub struct PermissionDto {
    pub id: Key<Permission>,
    pub resource: Resource,
    pub actions: Actions,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(output)]
pub struct RoleWithPermissionsDto {
    #[serde(flatten)]
    pub role: RoleDto,
    pub permissions: Vec<PermissionDto>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddRoleDto {
    #[validate(custom = "identifier")]
    pub code: String,
    #[validate(length(min = 4, max = 64))]
    pub friendly_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddPermissionDto {
    pub resource: Resource,
    pub actions: Actions,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct UpdateRoleDto {
    #[validate(length(min = 4, max = 64))]
    pub friendly_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddAccountToRoleDto {
    pub user_id: Key<User>,
    pub account_id: Key<Account>,
}

pub type RemoveAccountFromRoleDto = AddAccountToRoleDto;
