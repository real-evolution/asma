use aide::OperationIo;
use common_validation::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct UserCredentials {
    #[validate(custom = "username")]
    pub account_name: String,
    #[validate(custom = "username")]
    pub username: String,
    pub device_identifier: String,
    pub password: String,
}

#[derive(Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(output)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}
