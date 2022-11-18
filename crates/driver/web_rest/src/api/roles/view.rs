use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, RoleKey};
use kernel_repositories::auth::RolesRepo;

use crate::api::dtos::roles::{PermissionDto, RoleDto};
use crate::error::ApiResult;
use crate::extractors::di::Dep;
use crate::util::claims::Claims;

#[utoipa::path(
    get,
    path = "/api/roles/{id}",
    responses(
        (status = 200, description = "Role with `id", body = RoleDto),
        (status = 404, description = "No roles with `id` were found"),
    ),
)]
pub async fn get_by_id(
    claims: Claims,
    Path(id): Path<RoleKey>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Json<RoleDto>> {
    claims.require_any_role_with_permission(
        vec![KnownRoles::Root.into(), KnownRoles::Admin.into()],
        (Resource::Roles, Action::View.into()),
    )?;

    let role = roles_repo.get(&id).await?;
    let permissions: Vec<PermissionDto> = roles_repo
        .get_permissions_of(&role.id)
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect_vec();

    Ok(Json(RoleDto { role, permissions }))
}
