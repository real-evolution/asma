use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{
    auth::{InsertRole, RolesRepo, UpdateRole},
    error::{RepoError, RepoResult},
};
use ormx::{Delete, Patch, Table};
use shaku::Component;
use tracing::warn;

use crate::{
    database::SqlxDatabaseConnection,
    models::auth::{
        permission::PermissionModel,
        role::{AccountRoleModel, RoleModel, UpdateSessionModel},
    },
    util::error::map_sqlx_error,
};

#[derive(Component)]
#[shaku(interface = RolesRepo)]
pub struct SqlxRolesRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl RolesRepo for SqlxRolesRepo {
    async fn get(&self, id: &Key<Role>) -> RepoResult<Role> {
        Ok(RoleModel::get(self.db.get(), id.value())
            .await
            .map_err(map_sqlx_error)?
            .into())
    }

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<Role>> {
        Ok(sqlx::query_as!(
            RoleModel,
            r#"
            SELECT * FROM roles
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            pagination.0,
            pagination.1 as i64
        )
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?
        .into_iter()
        .map(|u| u.into())
        .collect())
    }

    async fn get_permissions_of(
        &self,
        role_id: &Key<Role>,
    ) -> RepoResult<Vec<Permission>> {
        Ok(PermissionModel::by_role(self.db.get(), role_id.value_ref())
            .await
            .map_err(map_sqlx_error)?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &Key<Account>,
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
            account_id.value_ref()
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

    async fn create(&self, insert: InsertRole) -> RepoResult<Key<Role>> {
        Ok(RoleModel::insert(
            self.db.acquire().await?.as_mut(),
            crate::models::auth::role::InsertRoleModel {
                code: insert.code,
                friendly_name: insert.friendly_name,
                is_active: true,
            },
        )
        .await
        .map_err(map_sqlx_error)?
        .id
        .into())
    }

    async fn update(
        &self,
        role_id: &Key<Role>,
        update: UpdateRole,
    ) -> RepoResult<()> {
        Ok(UpdateSessionModel {
            friendly_name: update.friendly_name,
            updated_at: Utc::now(),
        }
        .patch_row(self.db.get(), role_id.value())
        .await
        .map_err(map_sqlx_error)?)
    }

    async fn remove(&self, role_id: &Key<Role>) -> RepoResult<()> {
        Ok(RoleModel::delete_row(self.db.get(), role_id.value())
            .await
            .map_err(map_sqlx_error)?)
    }

    async fn add_permission(
        &self,
        role_id: &Key<Role>,
        resource: Resource,
        actions: Actions,
    ) -> RepoResult<Key<Permission>> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM permissions
                WHERE resource = $1 AND actions = $2 AND role_id = $3
            )"#,
            resource as i64,
            actions.inner(),
            role_id.value_ref()
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        if exists.unwrap_or(false) {
            return Err(RepoError::DuplicateValue(format!(
                "permission {resource}:{actions:?} was already added to role #{role_id:?}"
            )));
        }

        Ok(PermissionModel::insert(
            self.db.acquire().await?.as_mut(),
            crate::models::auth::permission::InsertPermissionModel {
                role_id: role_id.value(),
                actions,
                resource,
            },
        )
        .await
        .map_err(map_sqlx_error)?
        .id
        .into())
    }

    async fn remove_permission(
        &self,
        role_id: &Key<Role>,
        permission_id: &Key<Permission>,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"DELETE FROM permissions WHERE id = $1 AND role_id = $2"#,
            permission_id.value_ref(),
            role_id.value_ref(),
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn add_to(
        &self,
        account_id: &Key<Account>,
        role_id: &Key<Role>,
    ) -> RepoResult<()> {
        AccountRoleModel::insert(
            self.db.acquire().await?.as_mut(),
            crate::models::auth::role::InsertAccountRoleModel {
                account_id: account_id.value(),
                role_id: role_id.value(),
                is_active: true,
            },
        )
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn remove_from(
        &self,
        account_id: &Key<Account>,
        role_id: &Key<Role>,
    ) -> RepoResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM account_roles
            WHERE account_id = $1 AND role_id = $2
            "#,
            account_id.value_ref(),
            role_id.value_ref(),
        )
        .execute(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}
