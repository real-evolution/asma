use chrono::{DateTime, Utc};
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::InsertRole;
use mapper::Mapper;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleDto {
    #[serde(flatten)]
    pub role: Role,
}

#[derive(Debug, Deserialize, Mapper, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[from(Permission)]
pub struct PermissionDto {
    pub id: PermissionKey,
    pub resource: Resource,
    pub actions: Actions,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RoleWithPermissionsDto {
    #[serde(flatten)]
    pub role: RoleDto,
    pub permissions: Vec<PermissionDto>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleDto {
    #[validate(length(min = 4))]
    pub code: String,
    pub friendly_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddPermissionDto {
    pub resource: Resource,
    pub actions: Actions,
}

impl Into<InsertRole> for AddRoleDto {
    fn into(self) -> InsertRole {
        InsertRole::new(self.code, self.friendly_name)
    }
}
