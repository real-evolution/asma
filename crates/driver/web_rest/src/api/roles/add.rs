use kernel_entities::entities::auth::{Action, KnownRoles, Resource, RoleKey};
use kernel_repositories::auth::RolesRepo;

use crate::{
    api::dtos::roles::AddRoleDto,
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{claims::Claims, response::Created},
};

#[utoipa::path(
    post,
    path = "/api/roles",
    responses((status = 201, description = "Role created", body = RoleKey)),
)]
pub async fn add(
    claims: Claims,
    ValidatedJson(form): ValidatedJson<AddRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Created<RoleKey>> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Roles, Action::Add),
    )?;

    let id = roles_repo.create(form.into()).await?;

    Ok(Created("/api/roles", id))
}
