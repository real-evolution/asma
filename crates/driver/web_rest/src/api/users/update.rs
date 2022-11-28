use axum::extract::Path;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, User};
use kernel_entities::traits::Key;
use kernel_repositories::auth::UsersRepo;

use super::dtos::UpdateUserDto;
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::claims::Claims,
};

#[utoipa::path(
    patch,
    path = "/api/users/{user_id}",
    request_body = UpdateUserDto,
    responses((status = 200, description = "User updated")),
    params(
        (
            "user_id" = Key<User>,
            Path,
            description = "Id of the user to be updated"
        ),
    )
)]
pub async fn update(
    claims: Claims,
    user_id: Path<Key<User>>,
    ValidatedJson(form): ValidatedJson<UpdateUserDto>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Users, Action::Modify),
    )?;

    Ok(users_repo
        .set_display_name(&user_id, form.display_name)
        .await?)
}
