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

    async fn set_password_hash(
        &self,
        id: &Key<Account>,
        value: String,
    ) -> RepoResult<()>;

    async fn set_holder_name(
        &self,
        id: &Key<Account>,
        value: Option<String>,
    ) -> RepoResult<()>;

    async fn set_state(
        &self,
        id: &Key<Account>,
        value: AccountState,
    ) -> RepoResult<()>;
}

#[derive(Constructor, Debug)]
pub struct InsertAccount {
    pub user_id: Key<User>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
}
