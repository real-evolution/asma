use common_validation::*;
use derive_more::Constructor;
use kernel_entities::entities::auth::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Constructor, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    #[serde(flatten)]
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddUserDto {
    #[validate(custom = "username")]
    pub username: String,
    #[validate(length(min = 4, max = 32))]
    pub display_name: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDto {
    #[validate(length(min = 4, max = 32))]
    pub display_name: String,
}
