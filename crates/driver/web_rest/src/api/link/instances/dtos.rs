use aide::OperationIo;
use chrono::{DateTime, Utc};
use kernel_entities::{
    entities::{
        comm::Chat,
        link::{Channel, Instance},
    },
    traits::Key,
};
use kernel_repositories::link::UpdateInstance;
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Instance)]
#[aide(output)]
pub struct InstanceDto {
    pub id: Key<Instance>,
    pub platform_identifier: i64,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub phone_number: Option<String>,
    pub last_active: Option<DateTime<Utc>>,
    pub chat_id: Key<Chat>,
    pub channel_id: Key<Channel>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

