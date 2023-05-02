use chrono::{DateTime, Utc};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{
    auth::{AccountsRepo, InsertAccount},
    error::RepoResult,
    traits::*,
};
use ormx::{Delete, Patch, Table};
use proc_macros::Repo;

use crate::{database::SqlxPool, util::error::map_sqlx_error, *};

#[derive(Repo)]
#[repo(
    table = "accounts",
    read(entity = "Account", model = "models::AccountModel"),
    insert(entity = "InsertAccount", model = "models::InsertAccountModel")
)]
pub(crate) struct SqlxAccountsRepo(pub SqlxPool);

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
            .fetch_one(self.0.get())
            .await
        )
    }

    async fn get_count_for(&self, user_id: &Key<User>) -> RepoResult<usize> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(id) FROM accounts
            WHERE user_id = $1"#,
            user_id.value_ref()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(count.unwrap_or(0) as usize)
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
                INNER JOIN account_roles
                        ON account_roles.account_id = accounts.id
                INNER JOIN roles
                        ON account_roles.role_id = roles.id
                     WHERE roles.id                  = $1 AND
                           account_roles.created_at <= $2
                  ORDER BY account_roles.created_at DESC
                     LIMIT $3
                "#,
                role_id.value_ref(),
                before,
                limit as i64,
            )
            .fetch_all(self.0.get())
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
                INNER JOIN account_roles
                        ON account_roles.account_id = accounts.id
                INNER JOIN roles
                        ON account_roles.role_id = roles.id
                     WHERE roles.id                  = $1 AND
                           accounts.user_id          = $2 AND
                           account_roles.created_at <= $3
                  ORDER BY account_roles.created_at DESC
                     LIMIT $4
                "#,
                role_id.value_ref(),
                user_id.value_ref(),
                before,
                limit as i64,
            )
            .fetch_all(self.0.get())
            .await
        )
    }

    async fn exists_with_name_for(
        &self,
        user_id: &Key<User>,
        account_name: &str,
    ) -> RepoResult<bool> {
        Ok(sqlx::query_scalar!(
            r#"SELECT EXISTS (
                SELECT 1 FROM accounts
                WHERE user_id = $1 AND account_name = $2
            )"#,
            user_id.value_ref(),
            account_name
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?
        .unwrap_or(false))
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
            .patch_row(self.0.get(), id.value())
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
            .patch_row(self.0.get(), id.value())
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
            .patch_row(self.0.get(), id.value())
            .await
        )
    }
}

#[async_trait::async_trait]
impl ChildRepo<User> for SqlxAccountsRepo {
    async fn get_paginated_of(
        &self,
        user_id: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::AccountModel,
                r#"
                  SELECT *
                    FROM accounts
                   WHERE user_id     = $1 AND
                         created_at <= $2
                ORDER BY created_at DESC
                   LIMIT $3
                "#,
                user_id.value_ref(),
                before,
                limit as i64
            )
            .fetch_all(self.0.get())
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
                r#"
                  SELECT *
                    FROM accounts
                   WHERE id      = $1 AND
                         user_id = $2
                ORDER BY created_at DESC"#,
                id.value_ref(),
                user_id.value_ref()
            )
            .fetch_one(self.0.get())
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
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

#[async_trait::async_trait()]
impl StatsRepo<User> for SqlxAccountsRepo {
    async fn get_stats_for(
        &self,
        parent_key: &Key<User>,
    ) -> RepoResult<StatsPair> {
        sqlx::query!(
            r#"
            SELECT
                COUNT(id) AS "total!",
                (
                    SELECT COUNT(id) FROM accounts
                    WHERE user_id = $1 AND state = $2
                ) AS "active!"
            FROM accounts
            WHERE user_id = $1
            "#,
            parent_key.value_ref(),
            AccountState::Active.repr()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)
        .map(|r| StatsPair::new(r.total as u64, r.active as u64))
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
        #[ormx(default)]
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

    impl From<InsertAccount> for InsertAccountModel {
        fn from(val: InsertAccount) -> Self {
            InsertAccountModel {
                user_id: val.user_id.into(),
                account_name: val.account_name,
                holder_name: val.holder_name,
                password_hash: val.password_hash,
                state: val.state.into(),
            }
        }
    }

    generate_mapping!(Account, AccountModel, 8);
}
