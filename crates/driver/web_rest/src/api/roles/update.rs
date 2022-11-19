use axum::extract::Path;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, RoleKey};
use kernel_repositories::auth::{RolesRepo, UpdateRole};

use crate::{
    api::dtos::roles::UpdateRoleDto,
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::claims::Claims,
};

#[utoipa::path(
    patch,
    path = "/api/roles/{role_id}",
    request_body = UpdateRoleDto,
    responses((status = 200, description = "Role updated")),
    params(
        (
            "role_id" = RoleKey,
            Path,
            description = "Id of the role to be updated"
        ),
    )
)]
pub async fn update(
    claims: Claims,
    Path(role_id): Path<RoleKey>,
    ValidatedJson(form): ValidatedJson<UpdateRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Roles, Action::Modify),
    )?;

    roles_repo
        .update(&role_id, UpdateRole::new(form.friendly_name))
        .await?;

    Ok(())
}
