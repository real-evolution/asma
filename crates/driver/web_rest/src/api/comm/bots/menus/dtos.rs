use aide::OperationIo;
use chrono::{DateTime, Utc};
use kernel_entities::{
    entities::comm::{Bot, Menu, TriggerMatchingStrategy},
    traits::Key,
};
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Menu)]
#[aide(output)]
pub struct MenuDto {
    pub id: Key<Menu>,
    pub title: String,
    pub content: Option<String>,
    pub menu_trigger: String,
    pub matching_strategy: TriggerMatchingStrategy,
    pub is_active: bool,
    pub parent_menu_id: Key<Menu>,
    pub bot_id: Key<Bot>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddMenuDto {
    pub title: String,
    pub content: Option<String>,
    pub menu_trigger: String,
    pub matching_strategy: TriggerMatchingStrategy,
    pub is_active: bool,
    pub parent_menu_id: Option<Key<Menu>>,
    pub bot_id: Key<Bot>,
}
