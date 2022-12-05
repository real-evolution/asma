use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, Role};
use kernel_entities::traits::Key;
use kernel_repositories::auth::RolesRepo;

use super::dtos::{PermissionDto, RoleDto, RoleWithPermissionsDto};
use crate::api::dtos::pagination::Pagination;
use crate::error::ApiResult;
use crate::extractors::di::Dep;
use crate::extractors::validated_query::ValidatedQuery;
use crate::util::claims::Claims;

pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Json<Vec<RoleDto>>> {
    claims
        .in_role_with(KnownRoles::Admin, &[(Resource::Roles, Action::View)])?;

    let roles = roles_repo
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|r| RoleDto { role: r })
        .collect_vec();

    Ok(Json(roles))
}

pub async fn get_by_id(
    claims: Claims,
    role_id: Path<Key<Role>>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Json<RoleWithPermissionsDto>> {
    claims.can(&[(Resource::Roles, Action::View)])?;

    let role = roles_repo.get(&role_id).await?;
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

    claims: Claims,
    role_id: Path<Key<Role>>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Json<RoleWithPermissionsDto>> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[
            (Resource::Roles, Action::View),
            (Resource::Accounts, Action::View),
        ],
    )?;

    let role = roles_repo.get(&role_id).await?;
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
