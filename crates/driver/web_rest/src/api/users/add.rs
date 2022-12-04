use kernel_entities::entities::auth::*;
use kernel_repositories::auth::{InsertUser, UsersRepo};

use super::dtos::AddUserDto;
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{
        claims::Claims,
        response::{Created, EntityCreated},
    },
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
) -> ApiResult<EntityCreated<User>> {
    claims
        .in_role_with(KnownRoles::Admin, &[(Resource::Users, Action::Add)])?;

    let user = users_repo
        .create(InsertUser::new(
            form.username,
            form.display_name,
            form.is_active,
        ))
        .await?;

    Ok(Created::new("/api/users", user).into())
}
