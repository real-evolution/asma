use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};
use shaku::Interface;

use crate::{
    error::RepoResult,
    traits::repo::{InsertRepo, Repo},
};

#[async_trait::async_trait]
pub trait AccountsRepo:
    Repo<Account> + InsertRepo<Account, InsertAccount> + Interface
{
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
