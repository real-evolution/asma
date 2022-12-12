use aide::OperationIo;
use chrono::{DateTime, Utc};
use common_validation::*;
use kernel_entities::{
    entities::auth::{Account, AccountState, User},
    traits::Key,
};
use mapper::Mapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Mapper, Serialize, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[from(Account)]
#[aide(output)]
pub struct AccountDto {
    pub id: Key<Account>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub state: AccountState,
    pub user_id: Key<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct AddAccountDto {
    #[validate(custom = "username")]
    pub account_name: String,
    #[validate(length(min = 4, max = 128))]
    pub holder_name: Option<String>,
    pub password: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, OperationIo, Validate)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct UpdateAccountPasswordDto {
    pub old_password: String,
    #[validate(length(min = 8, max = 64))]
    pub new_password: String,
}
