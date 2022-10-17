use std::ops::Deref;

use kernel_entities::entities::*;
use kernel_repositories::AccountsRepo;

use crate::{util::map_sqlx_error, SqlxRepo};

#[async_trait::async_trait]
impl AccountsRepo for SqlxRepo {
    async fn get_of_user_by_name(
        &self,
        user_id: &UserKey,
        account_name: &String,
    ) -> anyhow::Result<Account> {
        Ok(sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE user_id = $1 AND account_name = $2",
        )
        .bind(user_id)
        .bind(account_name)
        .fetch_one(self.deref())
        .await
        .map_err(map_sqlx_error)?)
    }
}
