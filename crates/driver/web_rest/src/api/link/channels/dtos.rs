use aide::OperationIo;
use chrono::{DateTime, Utc};
use common_validation::*;
use kernel_entities::{
    entities::{
        auth::User,
        link::{Channel, ChannelPlatform},
    },
    traits::Key,
};
use kernel_repositories::link::UpdateChannel;
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Channel)]
#[aide(output)]
pub struct ChannelDto {
    pub id: Key<Channel>,
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub max_instances: Option<i64>,
    pub user_id: Key<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddChannelDto {
    pub user_id: Key<User>,
    #[validate(length(min = 4, max = 32))]
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    #[validate(custom = "in_future")]
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema, OperationIo, Mapper)]
#[serde(rename_all = "camelCase")]
#[from(UpdateChannel)]
#[aide(input)]
pub struct UpdateChannelDto {
    #[validate(length(min = 4, max = 32))]
    pub name: String,
    pub api_key: String,
    #[validate(custom = "in_future")]
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
}
