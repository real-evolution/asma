use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait AccountsRepo:
    Repo<Entity = Account>
    + InsertRepo<InsertAccount>
    + ChildRepo<User>
    + Send
    + Sync
{
    async fn get_of_user_by_name(
        &self,
        user_id: &Key<User>,
        account_name: &str,
    ) -> RepoResult<Account>;

    async fn get_in_role(
        &self,
        role_id: &Key<Role>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Account>>;

    async fn get_in_role_for(
        &self,
        user_id: &Key<User>,
        role_id: &Key<Role>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Account>>;

    async fn set_holder_name(
        &self,
        id: &Key<Account>,
        value: Option<String>,
    ) -> RepoResult<()>;

    async fn set_password_hash(
        &self,
        id: &Key<Account>,
        value: String,
    ) -> RepoResult<()>;

    async fn set_state(
        &self,
        id: &Key<Account>,
        value: AccountState,
    ) -> RepoResult<()>;

    async fn exists_with_name_for(
        &self,
        user_id: &Key<User>,
        account_name: &str,
    ) -> RepoResult<bool>;
}

#[derive(Constructor, Debug)]
pub struct InsertAccount {
    pub user_id: Key<User>,
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub state: AccountState,
}
