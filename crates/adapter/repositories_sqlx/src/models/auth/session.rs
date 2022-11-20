use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities;
use kernel_entities::entities::auth::AccountKey;
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "sessions", id = id, insertable, deletable)]
pub struct Session {
    pub id: Uuid,
    #[ormx(get_optional)]
    pub device_identifier: String,
    pub agent: String,
    pub refresh_token: String,
    pub last_address: String,
    #[ormx(custom_type)]
    pub account_id: AccountKey,
    pub expires_at: Option<DateTime<Utc>>,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(entities::auth::Session, Session, 9);
