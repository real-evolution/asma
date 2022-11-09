use std::sync::Arc;

use kernel_entities::entities::*;
use kernel_repositories::{error::RepoResult, InsertRole, RolesRepo};
use shaku::Component;

use crate::{util::map_sqlx_error, SqlxDatabaseConnection};

#[derive(Component)]
#[shaku(interface = RolesRepo)]
pub struct SqlxRolesRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl RolesRepo for SqlxRolesRepo {
    async fn get_all(&self) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT * FROM roles")
            .fetch_all(self.db.get())
            .await
            .map_err(map_sqlx_error)?)
    }

    async fn get_account_roles(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>(
            r#"
            SELECT roles.*
                FROM roles
            INNER JOIN account_roles
                ON roles.role_id = account_roles.role_id AND
                   account_roles.account_id = ?"#,
        )
        .bind(account_id)
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn is_in_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<bool> {
        let ret = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT id FROM account_roles
                WHERE enabled = true AND account_id = $1 AND role_id = $2 
            )"#,
            account_id.0,
            role_id.0
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(ret.exists.unwrap_or(false))
    }

    async fn create(&self, insert: InsertRole) -> RepoResult<RoleKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO roles (code, friendly_name)
            VALUES ($1, $2)
            RETURNING id
            "#,
            insert.code,
            insert.friendly_name
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(RoleKey(id))
    }
}
