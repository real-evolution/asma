use std::collections::HashMap;

use chrono::Utc;
use itertools::Itertools;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{auth::*, error::*, traits::*};
use ormx::{Delete, Patch, Table};
use proc_macros::Repo;
use tracing::warn;

use crate::{
    database::SqlxPool,
    sqlx_ok,
    sqlx_vec_ok,
    util::error::map_sqlx_error,
};

#[derive(Repo)]
#[repo(
    table = "roles",
    read(entity = "Role", model = "models::RoleModel"),
    insert(entity = "InsertRole", model = "models::InsertRoleModel")
)]
pub(crate) struct SqlxRolesRepo(pub SqlxPool);

#[async_trait::async_trait]
impl RolesRepo for SqlxRolesRepo {
    async fn get_permissions_of(
        &self,
        role_id: &Key<Role>,
    ) -> RepoResult<Vec<Permission>> {
        sqlx_vec_ok!(
            models::PermissionModel::by_role(
                self.0.get(),
                role_id.value_ref(),
            )
            .await
        )
    }

    async fn get_roles_with_permissions_for(
        &self,
        account_id: &Key<Account>,
    ) -> RepoResult<HashMap<String, Vec<(Resource, Actions)>>> {
        let items = sqlx::query_as!(
            models::RolePermissionPair,
            r#"
               SELECT roles.code,
                      permissions.resource as "resource?",
                      permissions.actions as "actions?"
                 FROM roles
                 JOIN account_roles
                   ON account_roles.account_id = $1
            LEFT JOIN permissions
                   ON permissions.role_id = account_roles.role_id
                WHERE roles.is_active = TRUE AND account_roles.is_active = TRUE
            "#,
            account_id.value_ref()
        )
        .fetch_all(self.0.get())
        .await
        .map_err(map_sqlx_error)?
        .into_iter()
        .map(|i| {
            let (Some(res), Some(act)) = (i.resource, i.actions) else {
                return (i.code, None)
            };

            if let Resource::Unknown = Resource::from(res) {
                warn!("unknown resource with code `{}`", res);
            }

            (i.code, Some((Resource::from(res), Actions::from(act))))
        })
        .into_group_map()
        .into_iter()
        .map(|(code, perms)| (code, perms.into_iter().flatten().collect()))
        .collect();

        Ok(items)
    }

    async fn set_friendly_name(
        &self,
        role_id: &Key<Role>,
        value: Option<String>,
    ) -> RepoResult<()> {
        sqlx_ok!(
            models::UpdateRoleModel {
                friendly_name: value,
                updated_at: Utc::now(),
            }
            .patch_row(self.0.get(), role_id.value())
            .await
        )
    }

    async fn add_permission(
        &self,
        role_id: &Key<Role>,
        resource: Resource,
        actions: Actions,
    ) -> RepoResult<Permission> {
        let resource: i64 = resource.into();
        let actions: i32 = actions.into();

        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM permissions
                WHERE resource = $1 AND actions = $2 AND role_id = $3
            )"#,
            resource,
            actions,
            role_id.value_ref()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        if exists.unwrap_or(false) {
            return Err(RepoError::DuplicateValue(format!(
                "permission {resource}:{actions:?} was already added to role \
                 #{role_id:?}"
            )));
        }

        sqlx_ok!(
            models::PermissionModel::insert(
                self.0.acquire().await?.as_mut(),
                models::InsertPermissionModel {
                    id: uuid::Uuid::new_v4(),
                    role_id: role_id.value(),
                    resource,
                    actions,
                },
            )
            .await
        )
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
        .execute(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn add_to(
        &self,
        account_id: &Key<Account>,
        role_id: &Key<Role>,
    ) -> RepoResult<()> {
        models::AccountRoleModel::insert(
            self.0.acquire().await?.as_mut(),
            models::InsertAccountRoleModel {
                id: uuid::Uuid::new_v4(),
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
        .execute(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::auth::*, traits::KeyType};
    use kernel_repositories::auth::InsertRole;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "roles", id = id, insertable, deletable)]
    pub struct RoleModel {
        pub id: KeyType,
        #[ormx(get_one, get_optional = by_code_optional)]
        pub code: String,
        #[ormx(set)]
        pub friendly_name: Option<String>,
        pub is_active: bool,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    #[derive(ormx::Patch)]
    #[ormx(table_name = "roles", table = RoleModel, id = "id")]
    pub struct UpdateRoleModel {
        pub friendly_name: Option<String>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "account_roles", id = id, insertable, deletable)]
    pub struct AccountRoleModel {
        pub id: KeyType,
        pub account_id: KeyType,
        pub role_id: KeyType,
        pub is_active: bool,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "permissions", id = id, insertable, deletable)]
    pub struct PermissionModel {
        pub id: KeyType,
        pub resource: i64,
        pub actions: i32,
        #[ormx(get_many=by_role)]
        pub role_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
    }

    impl From<InsertRole> for InsertRoleModel {
        fn from(val: InsertRole) -> Self {
            InsertRoleModel {
                id: uuid::Uuid::new_v4(),
                code: val.code,
                friendly_name: val.friendly_name,
                is_active: true,
            }
        }
    }

    #[derive(sqlx::FromRow)]
    pub struct RolePermissionPair {
        pub code: String,
        pub resource: Option<i64>,
        pub actions: Option<i32>,
    }

    generate_mapping!(Permission, PermissionModel, 5);
    generate_mapping!(Role, RoleModel, 6);
    generate_mapping!(AccountRole, AccountRoleModel, 6);
}
