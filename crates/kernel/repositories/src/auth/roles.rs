use std::collections::HashMap;

use derive_more::Constructor;
use kernel_entities::entities::auth::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait RolesRepo: Interface {
    async fn get_all(&self) -> RepoResult<Vec<Role>>;

    async fn is_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<bool>;

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<HashMap<String, Vec<(Resource, Actions)>>>;

    async fn add_to_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()>;

    async fn remove_from_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()>;

    async fn create(&self, insert: InsertRole) -> RepoResult<RoleKey>;

    async fn add_permission_to(
        &self,
        role_id: &RoleKey,
        resouce: Resource,
        actions: Actions,
    ) -> RepoResult<PermissionKey>;

    async fn remove_permission_from(
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