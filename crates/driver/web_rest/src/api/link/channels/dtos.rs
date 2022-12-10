use chrono::{DateTime, Utc};
use common_validation::*;
use derive_more::Constructor;
use kernel_entities::{
    entities::{
        auth::User,
        link::{Channel, ChannelPlatform},
    },
    traits::Key,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Constructor, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDto {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Constructor, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
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
