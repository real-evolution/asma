use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derive_more::Constructor;
use kernel_entities::entities::auth::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait RolesRepo: Interface {
    async fn get(&self, id: &RoleKey) -> RepoResult<Role>;

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<Role>>;

    async fn get_permissions_of(
        &self,
        role_id: &RoleKey,
    ) -> RepoResult<Vec<Permission>>;

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<HashMap<String, Vec<(Resource, Actions)>>>;

    async fn is_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<bool>;

    async fn create(&self, insert: InsertRole) -> RepoResult<RoleKey>;

    async fn add_to(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()>;

    async fn remove_from(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()>;

    async fn add_permission(
        &self,
        role_id: &RoleKey,
        resouce: Resource,
        actions: Actions,
    ) -> RepoResult<PermissionKey>;

    async fn remove_permission(
        &self,
        role_id: &RoleKey,
        permission_id: &PermissionKey,
    ) -> RepoResult<()>;
}

#[derive(Constructor)]
pub struct InsertRole {
    pub code: String,
    pub friendly_name: Option<String>,
}
