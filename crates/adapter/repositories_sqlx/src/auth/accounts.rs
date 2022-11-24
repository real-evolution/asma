use std::sync::Arc;

use adapter_proc_macros::Repo;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::{AccountsRepo, InsertAccount};
use kernel_repositories::error::RepoResult;
use kernel_repositories::traits::*;
use ormx::{Delete, Table};
use shaku::Component;

use crate::database::SqlxDatabaseConnection;
use crate::sqlx_ok;
use crate::util::error::map_sqlx_error;

#[derive(Component, Repo)]
#[repo(
    table = "accounts",
    read(entity = "Account", model = "models::AccountModel"),
    insert(entity = "InsertAccount", model = "models::InsertAccountModel")
)]
#[shaku(interface = AccountsRepo)]
pub struct SqlxAccountsRepo {
    #[shaku(inject)]
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
        pub holder_name: Option<String>,
        pub password_hash: String,
        pub state: i32,
        pub user_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
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
