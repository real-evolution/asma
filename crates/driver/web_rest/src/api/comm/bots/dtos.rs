use aide::OperationIo;
use chrono::{DateTime, Utc};
use kernel_entities::{entities::{comm::Bot, auth::User}, traits::Key};
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Bot)]
#[aide(output)]
pub struct BotDto {
    pub id: Key<Bot>,
    pub name: String,
    pub is_active: bool,
    pub user_id: Key<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddBotDto {
    pub name: String,
    pub is_active: bool,
    pub user_id: Key<User>,
}
