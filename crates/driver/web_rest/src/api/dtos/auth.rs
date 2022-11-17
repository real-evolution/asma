use common_validation::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(ToSchema, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    #[validate(custom = "username")]
    pub account_name: String,
    #[validate(custom = "username")]
    pub username: String,
    pub device_identifier: String,
    pub password: String,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}
