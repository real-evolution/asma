use chrono::{DateTime, Utc};
use common_validation::*;
use derive_more::Constructor;
use kernel_entities::{
    entities::auth::{Account, AccountState, User},
    traits::Key,
};
use mapper::Mapper;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Constructor, Debug, Deserialize, Mapper, Serialize)]
#[serde(rename_all = "camelCase")]
#[from(Account)]
pub struct AccountDto {
    pub id: Key<Account>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
    pub user_id: Key<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Constructor, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddAccountDto {
    #[validate(custom = "username")]
    pub account_name: String,
    #[validate(length(min = 4, max = 128))]
    pub holder_name: Option<String>,
    pub password: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountPasswordDto {
    pub old_password: String,
    #[validate(length(min = 8, max = 64))]
    pub new_password: String,
}
