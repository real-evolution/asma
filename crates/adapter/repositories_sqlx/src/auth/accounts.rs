use std::sync::Arc;

use adapter_proc_macros::Repo;
use chrono::{DateTime, Utc};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::{AccountsRepo, InsertAccount};
use kernel_repositories::error::RepoResult;
use kernel_repositories::traits::*;
use ormx::{Delete, Patch, Table};

use crate::database::SqlxDatabaseConnection;
use crate::util::error::map_sqlx_error;
use crate::{sqlx_ok, sqlx_vec_ok};

#[derive(Repo)]
#[repo(
    table = "accounts",
    read(entity = "Account", model = "models::AccountModel"),
    insert(entity = "InsertAccount", model = "models::InsertAccountModel")
)]
pub struct SqlxAccountsRepo {
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl AccountsRepo for SqlxAccountsRepo {
    async fn get_of_user_by_name(
        &self,
        user_id: &Key<User>,
        account_name: &str,
    ) -> RepoResult<Account> {
        sqlx_ok!(
            sqlx::query_as!(
                models::AccountModel,
                r#"
                SELECT * FROM accounts
                WHERE user_id = $1 AND account_name = $2
                "#,
                user_id.value_ref(),
                account_name,
            )
            .fetch_one(self.db.get())
            .await
        )
    }

    async fn set_password_hash(
        &self,
        id: &Key<Account>,
        value: String,
    ) -> RepoResult<()> {
        sqlx_ok!(
            models::UpdateAccountPasswordModel {
                password_hash: value,
                updated_at: Utc::now()
            }
            .patch_row(self.db.get(), id.value())
            .await
        )
    }

    async fn set_holder_name(
        &self,
        id: &Key<Account>,
        value: Option<String>,
    ) -> RepoResult<()> {
        sqlx_ok!(
            models::UpdateAccountHolderNameModel {
                holder_name: value,
                updated_at: Utc::now()
            }
            .patch_row(self.db.get(), id.value())
            .await
        )
    }

    async fn set_state(
        &self,
        id: &Key<Account>,
        value: AccountState,
    ) -> RepoResult<()> {
        sqlx_ok!(
            models::UpdateAccountStateModel {
                state: value.into(),
                updated_at: Utc::now()
            }
            .patch_row(self.db.get(), id.value())
            .await
        )
    }

    async fn get_in_role(
        &self,
        role_id: &Key<Role>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Account>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::AccountModel,
                r#"
                SELECT accounts.*
                FROM accounts
                INNER JOIN account_roles ON role_id = $1
                WHERE account_roles.created_at <= $2
                ORDER BY account_roles.created_at
                LIMIT $3"#,
                role_id.value_ref(),
                before,
                limit as i64,
            )
            .fetch_all(self.db.get())
            .await
        )
    }

    async fn get_in_role_for(
        &self,
        user_id: &Key<User>,
        role_id: &Key<Role>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Account>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::AccountModel,
                r#"
                SELECT accounts.*
                FROM accounts
                INNER JOIN account_roles ON role_id = $1
                WHERE accounts.user_id = $2 AND account_roles.created_at <= $3
                ORDER BY account_roles.created_at
                LIMIT $4"#,
                role_id.value_ref(),
                user_id.value_ref(),
                before,
                limit as i64,
            )
            .fetch_all(self.db.get())
            .await
        )
    }
}

#[async_trait::async_trait]
impl ChildRepo<Account, User> for SqlxAccountsRepo {
    async fn get_paginated_of(
        &self,
        user_id: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Account>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::AccountModel,
                r#"
                SELECT * FROM accounts
                WHERE user_id = $1 AND created_at <= $2
                ORDER BY created_at
                LIMIT $3
                "#,
                user_id.value_ref(),
                before,
                limit as i64
            )
            .fetch_all(self.db.get())
            .await
        )
    }

    async fn get_of(
        &self,
        user_id: &Key<User>,
        id: &Key<Account>,
    ) -> RepoResult<Account> {
        sqlx_ok!(
            sqlx::query_as!(
                models::AccountModel,
                r#"SELECT * FROM accounts WHERE id = $1 AND user_id = $2"#,
                id.value_ref(),
                user_id.value_ref()
            )
            .fetch_one(self.db.get())
            .await
        )
    }

    async fn remove_of(
        &self,
        user_id: &Key<User>,
        id: &Key<Account>,
    ) -> RepoResult<()> {
        sqlx::query_as!(
            models::AccountModel,
            r#"DELETE FROM accounts WHERE id = $1 AND user_id = $2"#,
            id.value_ref(),
            user_id.value_ref()
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::auth::Account, traits::KeyType};
    use kernel_repositories::auth::InsertAccount;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "accounts", id = id, insertable, deletable)]
    pub struct AccountModel {
        pub id: KeyType,
        #[ormx(get_one)]
        pub account_name: String,
        #[ormx(set)]
        pub holder_name: Option<String>,
        #[ormx(set)]
        pub password_hash: String,
        #[ormx(set)]
        pub state: i32,
        #[ormx(get_many)]
        pub user_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    #[derive(ormx::Patch)]
    #[ormx(table_name = "accounts", table = AccountModel, id = "id")]
    pub struct UpdateAccountPasswordModel {
        pub password_hash: String,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(ormx::Patch)]
    #[ormx(table_name = "accounts", table = AccountModel, id = "id")]
    pub struct UpdateAccountHolderNameModel {
        pub holder_name: Option<String>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(ormx::Patch)]
    #[ormx(table_name = "accounts", table = AccountModel, id = "id")]
    pub struct UpdateAccountStateModel {
        pub state: i32,
        pub updated_at: DateTime<Utc>,
    }

    impl Into<InsertAccountModel> for InsertAccount {
        fn into(self) -> InsertAccountModel {
            InsertAccountModel {
                id: uuid::Uuid::new_v4(),
                user_id: self.user_id.into(),
                account_name: self.account_name,
                holder_name: self.holder_name,
                password_hash: self.password_hash,
                state: self.state.into(),
            }
        }
    }

    generate_mapping!(Account, AccountModel, 8);
}
