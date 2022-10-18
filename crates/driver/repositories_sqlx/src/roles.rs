use std::ops::Deref;

use kernel_entities::entities::*;
use kernel_repositories::RolesRepo;

use crate::{util::map_sqlx_error, SqlxDatabase};

#[async_trait::async_trait]
impl RolesRepo for SqlxDatabase {
    async fn get_all(&self) -> anyhow::Result<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT * FROM roles")
            .fetch_all(self.deref())
            .await
            .map_err(map_sqlx_error)?)
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
        .await
        .map_err(map_sqlx_error)?)
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
        .await
        .map_err(map_sqlx_error)?;

        Ok(match ret {
            Some(_) => true,
            None => false,
        })
    }
}
