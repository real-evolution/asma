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

pub async fn update(
    claims: Claims,
    user_id: Path<Key<User>>,
    ValidatedJson(form): ValidatedJson<UpdateUserDto>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<()> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Users, Action::Modify)],
    )?;

    Ok(users_repo
        .set_display_name(&user_id, form.display_name)
        .await?)
}
