use kernel_entities::entities::*;

use crate::Repo;

#[async_trait::async_trait]
pub trait AccountsRepo: Repo<Account, AccountKey> {
    async fn get_of_user_by_name(
        &self,
        user_id: &UserKey,
        account_name: &String,
    ) -> anyhow::Result<Account>;
}
