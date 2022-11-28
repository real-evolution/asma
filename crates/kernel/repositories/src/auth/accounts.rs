use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};
use shaku::Interface;

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait AccountsRepo:
    Repo<Account> + InsertRepo<Account, InsertAccount> + Interface
{
    async fn get_paginated_for(
        &self,
        user_id: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Account>>;

    async fn get_of_user_by_name(
        &self,
        user_id: &Key<User>,
        account_name: &str,
    ) -> RepoResult<Account>;
}

#[derive(Constructor, Debug)]
pub struct InsertAccount {
    pub user_id: Key<User>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
}
