use axum::extract::Path;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, User};
use kernel_entities::traits::Key;
use kernel_repositories::auth::UsersRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

pub async fn remove(
    claims: Claims,
    user_id: Path<Key<User>>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<()> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Users, Action::Remove)],
    )?;

    users_repo.remove(&user_id).await?;

    Ok(())
}
