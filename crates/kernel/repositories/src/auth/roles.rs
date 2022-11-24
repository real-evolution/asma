use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait RolesRepo: Interface {
    async fn get(&self, id: &Key<Role>) -> RepoResult<Role>;

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<Role>>;

    async fn get_permissions_of(
        &self,
        role_id: &Key<Role>,
    ) -> RepoResult<Vec<Permission>>;

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<HashMap<String, Vec<(Resource, Actions)>>>;

    async fn create(&self, insert: InsertRole) -> RepoResult<Key<Role>>;

    async fn update(
        &self,
        role_id: &Key<Role>,
        update: UpdateRole,
    ) -> RepoResult<()>;

    async fn remove(&self, role_id: &Key<Role>) -> RepoResult<()>;

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

#[derive(Constructor)]
pub struct UpdateRole {
    pub friendly_name: Option<String>,
}
