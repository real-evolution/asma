use std::sync::Arc;

use kernel_entities::entities::auth::*;
use kernel_repositories::{
    auth::{AccountsRepo, InsertAccount},
    error::RepoResult,
};
use shaku::Component;

use crate::{database::SqlxDatabaseConnection, util::map_sqlx_error};

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

    async fn create_for(
        &self,
        user_id: &UserKey,
        insert: InsertAccount,
    ) -> RepoResult<AccountKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO accounts (
                account_name,
                holder_name,
                password_hash,
                state,
                user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            insert.account_name,
            insert.holder_name,
            insert.password_hash,
            insert.state as i32,
            user_id.0
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(AccountKey(id))
    }

    async fn get_roles(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>(
            r#"
            SELECT roles.* FROM roles
            INNER JOIN account_roles
                ON roles.id = account_roles.role_id AND
                   account_roles.account_id = $1"#,
        )
        .bind(account_id)
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }
}
