use axum::extract::Path;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, Role};
use kernel_entities::traits::Key;
use kernel_repositories::auth::RolesRepo;

use super::dtos::UpdateRoleDto;
use crate::{
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
            "role_id" = Key<Role>,
            Path,
            description = "Id of the role to be updated"
        ),
    )
)]
pub async fn update(
    claims: Claims,
    Path(role_id): Path<Key<Role>>,
    ValidatedJson(form): ValidatedJson<UpdateRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Roles, Action::Modify),
    )?;

    roles_repo
        .set_friendly_name(&role_id, form.friendly_name)
        .await?;

    Ok(())
}
