use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, RoleKey};
use kernel_repositories::auth::RolesRepo;

use crate::api::dtos::pagination::Pagination;
use crate::api::dtos::roles::{PermissionDto, RoleDto, RoleWithPermissionsDto};
use crate::error::ApiResult;
use crate::extractors::di::Dep;
use crate::extractors::validated_query::ValidatedQuery;
use crate::util::claims::Claims;

#[utoipa::path(
    get,
    path = "/api/roles",
    responses(
        (status = 200, description = "All available roles", body = Vec<RoleDto>),
    ),
    params(Pagination)
)]
pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Json<Vec<RoleDto>>> {
    claims.require_any_role_with_permission(
        vec![KnownRoles::Root, KnownRoles::Admin],
        (Resource::Roles, Action::View | Action::Global),
    )?;

    let roles = roles_repo
        .get_all(pagination.into())
        .await?
        .into_iter()
        .map(|r| RoleDto { role: r })
        .collect_vec();

    Ok(Json(roles))
}

#[utoipa::path(
    get,
    path = "/api/roles/{id}",
    responses(
        (status = 200, description = "Role with `id", body = RoleWithPermissionsDto),
        (status = 404, description = "No roles with `id` were found"),
    ),
)]
pub async fn get_by_id(
    claims: Claims,
    Path(id): Path<RoleKey>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Json<RoleWithPermissionsDto>> {
    claims.require_any_role_with_permission(
        vec![KnownRoles::Root, KnownRoles::Admin],
        (Resource::Roles, Action::View | Action::Global),
    )?;

    let role = roles_repo.get(&id).await?;
    let permissions: Vec<PermissionDto> = roles_repo
        .get_permissions_of(&role.id)
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect_vec();

    Ok(Json(RoleWithPermissionsDto {
        role: RoleDto { role },
        permissions,
    }))
}
