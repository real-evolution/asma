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
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(output)]
pub struct ChannelDto {
    #[serde(flatten)]
    pub channel: Channel,
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
