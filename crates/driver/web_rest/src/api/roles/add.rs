use axum::{extract::Path, Json};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::{AccountsRepo, InsertRole, RolesRepo};

use super::dtos::{AddAccountToRoleDto, AddPermissionDto, AddRoleDto};
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{
        claims::Claims,
        response::{Created, EntityCreated},
    },
};

pub async fn add(
    claims: Claims,
    ValidatedJson(form): ValidatedJson<AddRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<EntityCreated<Role>> {
    claims
        .in_role_with(KnownRoles::Admin, &[(Resource::Roles, Action::Add)])?;

    let role = roles_repo
        .create(InsertRole::new(form.code, form.friendly_name))
        .await?;

    Ok(Created::new("/api/roles", role).into())
}

pub async fn add_permission(
    claims: Claims,
    role_id: Path<Key<Role>>,
    Json(form): Json<AddPermissionDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<EntityCreated<Permission>> {
    claims.in_role_with(
        KnownRoles::Root,
        &[
            (Resource::Roles, Action::Modify),
            (Resource::Permissions, Action::Add),
        ],
    )?;

    let permission = roles_repo
        .add_permission(&role_id, form.resource, form.actions)
        .await?;

    Ok(Created::new(
        format!("/api/roles/{}", role_id.0),
        permission,
    ))
}

pub async fn add_to(
    claims: Claims,
    role_id: Path<Key<Role>>,
    Json(form): Json<AddAccountToRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<()> {
    claims.in_role_with(
        KnownRoles::UserOwner,
        &[(Resource::Roles, Action::Modify)],
    )?;

    let role = roles_repo.get(&role_id).await?;
    claims.in_role(role.code.as_str())?;

    let account = accounts_repo.get(&form.account_id).await?;
    claims.of(&account.user_id)?;

    roles_repo.add_to(&account.id, &role.id).await?;

    Ok(())
}
