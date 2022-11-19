use axum::extract::Path;
use kernel_entities::entities::auth::{
    Action, KnownRoles, PermissionKey, Resource, RoleKey, UserKey,
};
use kernel_repositories::auth::{RolesRepo, UsersRepo};

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    responses((status = 200, description = "User removed")),
    responses((status = 404, description = "User not found")),
    params(
        ("user_id" = UserKey, Path, description = "Id of the user to remove"),
    )
)]
pub async fn remove(
    claims: Claims,
    Path(user_id): Path<UserKey>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permission(
        KnownRoles::Root | KnownRoles::Admin,
        (Resource::Users, Action::Remove),
    )?;

    users_repo.remove(&user_id).await?;

    Ok(())
}
