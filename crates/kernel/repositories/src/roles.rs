use kernel_entities::entities::*;

use crate::Repo;

#[async_trait::async_trait]
pub trait RolesRepo: Repo<Role, RoleKey> {
    async fn get_all(&self) -> anyhow::Result<Vec<Role>>;
    async fn get_account_roles(&self, account_id: &AccountKey) -> anyhow::Result<Vec<Role>>;
    async fn is_account_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> anyhow::Result<bool>;
}
