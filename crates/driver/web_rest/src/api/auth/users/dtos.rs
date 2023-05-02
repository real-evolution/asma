use aide::OperationIo;
use chrono::{DateTime, Utc};
use common_validation::*;
use kernel_entities::{entities::auth::User, traits::Key};
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Mapper, Deserialize, JsonSchema, Serialize, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(User)]
#[aide(output)]
pub struct UserDto {
    pub id: Key<User>,
    pub display_name: String,
    pub username: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddUserDto {
    #[validate(custom = "username")]
    pub username: String,
    #[validate(length(min = 4, max = 32))]
    pub display_name: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct UpdateUserDto {
    #[validate(length(min = 4, max = 32))]
    pub display_name: String,
}
