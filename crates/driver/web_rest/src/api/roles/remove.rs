use axum::extract::Path;
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
use kernel_repositories::auth::RolesRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

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
    Path(role_id): Path<Key<Role>>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Roles, Action::Remove),
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
    Path(role_id): Path<Key<Role>>,
    Path(permission_id): Path<Key<Permission>>,
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
