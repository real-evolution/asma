use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities::auth::Session;
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "sessions", id = id, insertable, deletable)]
pub struct SessionModel {
    #[ormx(default)]
    pub id: Uuid,
    #[ormx(get_optional)]
    pub device_identifier: String,
    pub agent: String,
    pub refresh_token: String,
    pub last_address: String,
    pub account_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

#[derive(ormx::Patch)]
#[ormx(table_name = "sessions", table = SessionModel, id = "id")]
pub struct UpdateSessionModel {
    pub last_address: String,
    pub agent: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(Session, SessionModel, 9);
