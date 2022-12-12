use aide::OperationIo;
use common_validation::*;
use kernel_entities::entities::auth::User;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, JsonSchema, Serialize, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(output)]
pub struct UserDto {
    #[serde(flatten)]
    pub user: User,
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
