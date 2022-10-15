use std::ops::Deref;

use kernel_entities::entities::*;
use kernel_repositories::AccountsRepo;

use crate::SqlxRepo;

#[async_trait::async_trait]
impl AccountsRepo for SqlxRepo {
    async fn get_of_user_by_name(
        &self,
        user_id: &UserKey,
        account_name: &String,
    ) -> anyhow::Result<Account> {
        Ok(sqlx::query_as::<_, Account>(
            r#"SELECT * FROM accounts WHERE user_id = ? AND account_name = ?"#,
        )
        .bind(user_id)
        .bind(account_name)
        .fetch_one(self.deref())
        .await?)
    }
}
