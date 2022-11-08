use std::sync::Arc;

use kernel_entities::entities::*;
use kernel_repositories::{error::RepoResult, AccountsRepo};
use shaku::Component;

use crate::{util::map_sqlx_error, SqlxDatabaseConnection};

#[derive(Component)]
#[shaku(interface = AccountsRepo)]
pub struct SqlxAccountsRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl AccountsRepo for SqlxAccountsRepo {
    async fn get_of_user_by_name(
        &self,
        user_id: &UserKey,
        account_name: &str,
    ) -> RepoResult<Account> {
        Ok(sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE user_id = $1 AND account_name = $2",
        )
        .bind(user_id)
        .bind(account_name)
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }
}
