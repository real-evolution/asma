use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;
use kernel_entities::entities::auth::*;
use kernel_repositories::{
    auth::{InsertRole, RolesRepo},
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
    async fn get_all(&self) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT * FROM roles")
            .fetch_all(self.db.get())
            .await
            .map_err(map_sqlx_error)?)
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

    async fn add_to_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()> {
        sqlx::query_scalar!(
            r#"
            INSERT INTO account_roles (account_id, role_id, enabled)
            VALUES ($1, $2, $3)
            "#,
            account_id.0,
            role_id.0,
            true
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn remove_from_role(
        &self,
        account_id: &AccountKey,
        role_id: &RoleKey,
    ) -> RepoResult<()> {
        sqlx::query_scalar!(
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

    async fn add_permission_to(
        &self,
        role_id: &RoleKey,
        resource: Resource,
        actions: Actions,
    ) -> RepoResult<PermissionKey> {
        let resource = resource as i64;
        let actions = actions.inner();

        let id = sqlx::query_scalar!(
            r#"
            SELECT id FROM permissions
            WHERE resource = $1 AND actions = $2 AND role_id = $3"#,
            resource,
            actions,
            role_id.0
        )
        .fetch_optional(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        if let Some(_) = id {
            return Err(RepoError::DuplicateValue(format!(
                "permission {resource}:{actions:b} was already added to role #{role_id:?}"
            )));
        }

        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO permissions (resource, actions, role_id)
            VALUES ($1, $2, $3)
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

    async fn remove_permission_from(
        &self,
        role_id: &RoleKey,
        permission_id: &PermissionKey,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"DELETE FROM permissions WHERE id = $1 AND role_id = $2"#,
            permission_id.0,
            role_id.0
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}
