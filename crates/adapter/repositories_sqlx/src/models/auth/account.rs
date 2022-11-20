use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_entities::entities;
use kernel_entities::entities::auth::{AccountState, UserKey};
use uuid::Uuid;

use crate::generate_mapping;

#[derive(Clone, Debug, From, Into, ormx::Table)]
#[ormx(table = "accounts", id = id, insertable, deletable)]
pub struct Account {
    #[ormx(custom_type)]
    pub id: Uuid,
    #[ormx(get_one)]
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    #[ormx(custom_type)]
    pub state: AccountState,
    #[ormx(custom_type)]
    pub user_id: UserKey,
    #[ormx(default)]
    pub created_at: DateTime<Utc>,
    #[ormx(default, set)]
    pub updated_at: DateTime<Utc>,
}

generate_mapping!(entities::auth::Account, Account, 8);