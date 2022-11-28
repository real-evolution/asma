use axum::extract::Path;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, User};
use kernel_entities::traits::Key;
use kernel_repositories::auth::UsersRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    responses((status = 200, description = "User removed")),
    responses((status = 404, description = "User not found")),
    params(
        ("user_id" = Key<User>, Path, description = "Id of the user to remove"),
    )
)]
pub async fn remove(
    claims: Claims,
    user_id: Path<Key<User>>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<()> {
    claims.require_any_role_with_permission(
        vec![KnownRoles::Root, KnownRoles::Admin],
        (Resource::Users, Action::Remove),
    )?;

    users_repo.remove(&user_id).await?;

    Ok(())
}
