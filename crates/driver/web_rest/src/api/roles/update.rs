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

pub async fn update(
    claims: Claims,
    role_id: Path<Key<Role>>,
    ValidatedJson(form): ValidatedJson<UpdateRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<()> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Roles, Action::Modify)],
    )?;

    roles_repo
        .set_friendly_name(&role_id, form.friendly_name)
        .await?;

    Ok(())
}
