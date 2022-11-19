use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use kernel_entities::entities::auth::*;
use kernel_repositories::{
    auth::{InsertRole, RolesRepo, UpdateRole},
    error::{RepoError, RepoResult},
};
use shaku::Component;
use tracing::warn;

use crate::{database::SqlxDatabaseConnection, util::map_sqlx_error};

#[derive(Component)]
#[shaku(interface = RolesRepo)]
pub struct SqlxRolesRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl RolesRepo for SqlxRolesRepo {
    async fn get(&self, id: &RoleKey) -> RepoResult<Role> {
        let role =
            sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = $1")
                .bind(id)
                .fetch_one(self.db.get())
                .await
                .map_err(map_sqlx_error)?;

        Ok(role)
    }

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>(
            r#"
            SELECT * FROM roles
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(pagination.0)
        .bind(pagination.1 as i64)
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn get_permissions_of(
        &self,
        role_id: &RoleKey,
    ) -> RepoResult<Vec<Permission>> {
        let permissions = sqlx::query_as::<_, Permission>(
            "SELECT * FROM permissions WHERE role_id = $1",
        )
        .bind(role_id)
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(permissions)
    }

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &AccountKey,
    ) -> RepoResult<HashMap<String, Vec<(Resource, Actions)>>> {
        let items = sqlx::query!(
            r#"
            SELECT roles.code, permissions.resource, permissions.actions
            FROM   roles
            JOIN   account_roles
              ON   account_roles.account_id = $1
            JOIN   permissions
              ON   permissions.role_id = account_roles.role_id
            "#,
            account_id.0
        )
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?
        .into_iter()
        .map(|i| {
            if let Some(res) = Resource::from_repr(i.resource) {
                Some((i.code, (res, Actions::from_bits(i.actions))))
            } else {
                warn!("unknown resource with code `{}`", i.resource);
                None
            }
        })
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .into_group_map();

        Ok(items)
    }

    async fn create(&self, insert: InsertRole) -> RepoResult<RoleKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT    INTO roles (code, friendly_name)
            VALUES    ($1, $2)
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

    async fn update(
        &self,
        role_id: &RoleKey,
        update: UpdateRole,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"UPDATE roles SET friendly_name = $1 WHERE id = $2"#,
            update.friendly_name,
            role_id.0
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn remove(&self, role_id: &RoleKey) -> RepoResult<()> {
        sqlx::query!(r#"DELETE FROM roles WHERE id = $1"#, role_id.0)
            .execute(self.db.get())
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn add_permission(
        &self,
        role_id: &RoleKey,
        resource: Resource,
        actions: Actions,
    ) -> RepoResult<PermissionKey> {
        let resource = resource as i64;
        let actions = actions.inner();

        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM permissions
                WHERE resource = $1 AND actions = $2 AND role_id = $3
            )"#,
            resource,
            actions,
            role_id.0
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        if exists.unwrap_or(false) {
            return Err(RepoError::DuplicateValue(format!(
                "permission {resource}:{actions:b} was already added to role #{role_id:?}"
            )));
        }

        let id = sqlx::query_scalar!(
            r#"
            INSERT    INTO permissions (resource, actions, role_id)
            VALUES    ($1, $2, $3)
            RETURNING id
            "#,
            resource,
            actions,
            role_id.0,
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(PermissionKey(id))
    }

    async fn remove_permission(
        &self,
        role_id: &RoleKey,
        permission_id: &PermissionKey,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"DELETE FROM permissions WHERE id = $1 AND role_id = $2"#,
            permission_id.0,
            role_id.0,
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn add_to(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO account_roles (account_id, role_id, is_active)
            VALUES ($1, $2, true)
            "#,
            account_id.0,
            role_id.0,
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn remove_from(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM account_roles
            WHERE account_id = $1 AND role_id = $2
            "#,
            account_id.0,
            role_id.0,
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}
