use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{
    entities::auth::{Account, AccountState, User},
    traits::Key,
};
use mapper::Mapper;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Constructor, Debug, Deserialize, Mapper, Serialize, ToSchema)]
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