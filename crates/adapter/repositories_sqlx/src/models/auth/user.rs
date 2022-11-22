use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities;
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table, sqlx::FromRow)]
#[ormx(table = "users", id = id, insertable, deletable)]
pub struct UserModel {
    pub id: Uuid,
    pub display_name: String,
    #[ormx(get_one(&str))]
    pub username: String,
    pub is_active: bool,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(entities::auth::User, UserModel, 6);
