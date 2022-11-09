use chrono::{DateTime, Utc};
use kernel_entities::entities::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait AccountsRepo: Interface {
    async fn get_of_user_by_name(
        &self,
        user_id: &UserKey,
        account_name: &str,
    ) -> RepoResult<Account>;

    async fn create_for(
        &self,
        user_id: &UserKey,
        insert: InsertAccount,
    ) -> RepoResult<AccountKey>;
}

#[derive(Debug)]
pub struct InsertAccount {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub is_active: bool,
    pub valid_until: Option<DateTime<Utc>>,
}

impl InsertAccount {
    pub fn new_active(
        account_name: String,
        holder_name: Option<String>,
        password_hash: String,
    ) -> Self {
        Self {
            account_name,
            holder_name,
            password_hash,
            valid_until: None,
            is_active: true,
        }
    }
}
