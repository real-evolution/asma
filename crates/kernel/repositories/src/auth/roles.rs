use std::collections::HashMap;

use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait RolesRepo:
    Repo<Entity = Role> + InsertRepo<Role, InsertRole> + Send + Sync
{
    async fn get_permissions_of(
        &self,
        role_id: &Key<Role>,
    ) -> RepoResult<Vec<Permission>>;

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<HashMap<String, Vec<(Resource, Actions)>>>;

    async fn set_friendly_name(
        &self,
        role_id: &Key<Role>,
        value: Option<String>,
    ) -> RepoResult<()>;

    async fn add_to(
        &self,
        account_id: &Key<Account>,
        role_id: &Key<Role>,
    ) -> RepoResult<()>;

    async fn remove_from(
        &self,
        account_id: &Key<Account>,
        role_id: &Key<Role>,
    ) -> RepoResult<()>;

    async fn add_permission(
        &self,
        role_id: &Key<Role>,
        resouce: Resource,
        actions: Actions,
    ) -> RepoResult<Permission>;

    async fn remove_permission(
        &self,
        role_id: &Key<Role>,
        permission_id: &Key<Permission>,
    ) -> RepoResult<()>;
}

#[derive(Constructor)]
pub struct InsertRole {
    pub code: String,
    pub friendly_name: Option<String>,
}
