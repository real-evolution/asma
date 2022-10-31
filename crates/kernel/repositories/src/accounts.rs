use crate::error::RepoResult;
use kernel_entities::entities::*;

use shaku::Interface;

#[async_trait::async_trait]
pub trait AccountsRepo: Interface {
    async fn get_of_user_by_name(
        &self,
        user_id: &UserKey,
        account_name: &str,
    ) -> RepoResult<Account>;
}
