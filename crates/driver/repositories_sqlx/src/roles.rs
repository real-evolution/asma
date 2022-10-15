use std::ops::Deref;

use kernel_entities::entities::*;
use kernel_repositories::RolesRepo;

use crate::SqlxRepo;

#[async_trait::async_trait]
impl RolesRepo for SqlxRepo {
    async fn get_all(&self) -> anyhow::Result<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT * FROM roles")
            .fetch_all(self.deref())
            .await?)
    }

    async fn get_account_roles(&self, account_id: &AccountKey) -> anyhow::Result<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>(
            r#"
            SELECT roles.*
                FROM roles
            INNER JOIN account_roles
                ON roles.role_id = account_roles.role_id AND
                   account_roles.account_id = ?"#,
        )
        .bind(account_id)
        .fetch_all(self.deref())
        .await?)
    }

    async fn is_account_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> anyhow::Result<bool> {
        let ret = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT account_roles.id
                    FROM account_roles
                WHERE enabled = true AND account_id = ? AND role_id = ? 
            )"#,
        )
        .bind(account_id)
        .bind(role_id)
        .fetch_optional(self.deref())
        .await?;

        Ok(match ret {
            Some(_) => true,
            None => false,
        })
    }
}
