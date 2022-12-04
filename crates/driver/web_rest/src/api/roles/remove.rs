use axum::{extract::Path, Json};
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
use kernel_repositories::auth::{RolesRepo, AccountsRepo};

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

use super::dtos::RemoveAccountFromRoleDto;

#[utoipa::path(
    delete,
    path = "/api/roles/{role_id}",
    responses((status = 200, description = "Role removed")),
    responses((status = 404, description = "Role not found")),
    params(
        ("role_id" = Key<Role>, Path, description = "Id of the role to remove"),
    )
)]
pub async fn remove(
    claims: Claims,
    role_id: Path<Key<Role>>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Roles, Action::Remove)],
    )?;

    roles_repo.remove(&role_id).await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/api/roles/{role_id}/permissions/{permission_id}",
    responses((status = 200, description = "Permission removed")),
    responses((status = 404, description = "Role not found")),
    responses((status = 404, description = "Permission not found")),
    params(
        (
            "role_id" = Key<Role>,
            Path,
            description = "Id of the role to remove the permission from"
        ),
        (
            "permission_id" = Key<Role>,
            Path,
            description = "Id of the permission to be removed"
        ),
    )
)]
pub async fn remove_permission(
    claims: Claims,
    role_id: Path<Key<Role>>,
    permission_id: Path<Key<Permission>>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[
            (Resource::Roles, Action::Modify),
            (Resource::Permissions, Action::Remove),
        ],
    )?;

    roles_repo
        .remove_permission(&role_id, &permission_id)
        .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/api/roles/{role_id}/accounts",
    request_body = AddAccountToRoleDto,
    responses(
        (status = 200, description = "Account removed"),
        (status = 404, description = "Role or account or user does not exist"),
    ),
    params(
        (
            "role_id" = Key<Role>,
            Path,
            description = "Id of the role to remove the account from"
        ),
    )
)]
pub async fn remove_from(
    claims: Claims,
    role_id: Path<Key<Role>>,
    Json(form): Json<RemoveAccountFromRoleDto>,
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

    roles_repo.remove_from(&account.id, &role.id).await?;

    Ok(())
}
