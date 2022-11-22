use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities::auth::{Actions, Permission, Resource};
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "permissions", id = id, insertable, deletable)]
pub struct PermissionModel {
    #[ormx(default)]
    pub id: Uuid,
    #[ormx(custom_type)]
    pub resource: Resource,
    #[ormx(custom_type)]
    pub actions: Actions,
    #[ormx(get_many=by_role)]
    pub role_id: Uuid,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
}

generate_mapping!(Permission, PermissionModel, 5);
