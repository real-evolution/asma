use kernel_entities::entities::*;

use crate::{Repo, error::RepoResult};

#[async_trait::async_trait]
pub trait RolesRepo: Repo<Role, RoleKey> {
    async fn get_all(&self) -> RepoResult<Vec<Role>>;
    async fn get_account_roles(&self, account_id: &AccountKey) -> RepoResult<Vec<Role>>;
    async fn is_account_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<bool>;
}
