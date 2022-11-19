use axum::extract::Path;
use kernel_entities::entities::auth::{
    Action, KnownRoles, PermissionKey, Resource, RoleKey,
};
use kernel_repositories::auth::RolesRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    delete,
    path = "/api/roles/{role_id}",
    responses((status = 200, description = "Role removed")),
    params(
        ("role_id" = RoleKey, Path, description = "Id of the role to remove"),
    )
)]
pub async fn remove(
    claims: Claims,
    Path(id): Path<RoleKey>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Roles, Action::Remove),
    )?;

    roles_repo.remove(&id).await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/api/roles/{role_id}/permissions/{permission_id}",
    responses((status = 200, description = "Permission removed")),
    params(
        (
            "role_id" = RoleKey,
            Path,
            description = "Id of the role to remove the permission from"
        ),
        (
            "permission_id" = RoleKey,
            Path,
            description = "Id of the permission to be removed"
        ),
    )
)]
pub async fn remove_permission(
    claims: Claims,
    Path(role_id): Path<RoleKey>,
    Path(permission_id): Path<PermissionKey>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permissions(
        KnownRoles::Root,
        vec![
            (Resource::Roles, Action::Modify),
            (Resource::Permissions, Action::Remove),
        ],
    )?;

    roles_repo
        .remove_permission(&role_id, &permission_id)
        .await?;

    Ok(())
}
