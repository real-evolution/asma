use axum::extract::Path;
use kernel_entities::entities::auth::{
    Action, KnownRoles, PermissionKey, Resource, RoleKey,
};
use kernel_repositories::auth::RolesRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    delete,
    path = "/api/roles/{role_id}/permissions/{permission_id}",
    responses((status = 200, description = "Permission removed")),
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
