use common_validation::*;
use derive_more::Constructor;
use kernel_entities::entities::auth::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Constructor, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    #[serde(flatten)]
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddUserDto {
    #[validate(custom = "username")]
    pub username: String,
    #[validate(length(min = 4, max = 32))]
    pub display_name: String,
    pub is_active: bool,
}
