use axum::{extract::Path, Json};
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
use kernel_repositories::auth::{RolesRepo, AccountsRepo};

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

use super::dtos::RemoveAccountFromRoleDto;

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
