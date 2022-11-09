use kernel_entities::entities::*;
use shaku::Interface;

use crate::error::RepoResult;

#[async_trait::async_trait]
pub trait RolesRepo: Interface {
    async fn get_all(&self) -> RepoResult<Vec<Role>>;
    async fn get_account_roles(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Role>>;

    async fn is_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<bool>;

    async fn add_to_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()>;

    async fn add_to_roles(
        &self,
        account_id: &AccountKey,
        role_ids: Vec<&RoleKey>,
    ) -> RepoResult<()>;

    async fn remove_from_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()>;

    async fn toggle_membership(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
        enabled: bool,
    ) -> RepoResult<()>;

    async fn create(&self, insert: InsertRole) -> RepoResult<RoleKey>;
}

pub struct InsertRole {
    pub code: String,
    pub friendly_name: Option<String>,
}
