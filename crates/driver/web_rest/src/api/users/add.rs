use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::{InsertUser, UsersRepo};

use super::dtos::AddUserDto;
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{claims::Claims, response::Created},
};

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = AddUserDto,
    responses((status = 201, description = "User created")),
)]
pub async fn add(
    claims: Claims,
    ValidatedJson(form): ValidatedJson<AddUserDto>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<Created<Key<User>>> {
    claims.require_any_role_with_permission(
        vec![KnownRoles::Root, KnownRoles::Admin],
        (Resource::Users, Action::Add),
    )?;

    let user = users_repo
        .create(InsertUser::new(
            form.username,
            form.display_name,
            form.is_active,
        ))
        .await?;

    Ok(Created("/api/users", user.id).into())
}
