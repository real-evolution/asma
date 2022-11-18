use chrono::{DateTime, Utc};
use kernel_entities::entities::auth::*;
use mapper::Mapper;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleDto {
    #[serde(flatten)]
    pub role: Role,
    pub permissions: Vec<PermissionDto>,
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
