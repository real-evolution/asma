use derive_more::Constructor;
use kernel_entities::entities::auth::*;
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

#[derive(Constructor, Debug)]
pub struct InsertAccount {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
}
