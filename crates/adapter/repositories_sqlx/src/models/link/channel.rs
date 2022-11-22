use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities::link::{Channel, ChannelPlatform};
use ormx::Table;
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, Table)]
#[ormx(table = "channels", id = id, insertable, deletable)]
pub struct ChannelModel {
    pub id: Uuid,
    pub name: String,
    #[ormx(custom_type)]
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    #[ormx(custom_type)]
    pub user_id: Uuid,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(Channel, ChannelModel, 9);
