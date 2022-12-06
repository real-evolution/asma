use axum::extract::State;
use axum::{extract::Path, Json};
use driver_web_common::state::AppState;
use itertools::Itertools;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, Role};
use kernel_entities::traits::Key;

use super::dtos::{PermissionDto, RoleDto, RoleWithPermissionsDto};
use crate::api::dtos::pagination::Pagination;
use crate::error::ApiResult;
use crate::util::claims::Claims;

pub async fn get_all(
    claims: Claims,
    pagination: Pagination,
    state: State<AppState>,
) -> ApiResult<Json<Vec<RoleDto>>> {
    claims
        .in_role_with(KnownRoles::Admin, &[(Resource::Roles, Action::View)])?;

    let roles = state
        .data
        .auth()
        .roles()
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect_vec();

    Ok(Json(roles))
}

pub async fn get_by_id(
    claims: Claims,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
) -> ApiResult<Json<RoleWithPermissionsDto>> {
    claims.can(&[(Resource::Roles, Action::View)])?;

    let role = state.data.auth().roles().get(&role_id).await?;
    let permissions: Vec<PermissionDto> = state
        .data
        .auth()
        .roles()
        .get_permissions_of(&role.id)
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect_vec();

    Ok(Json(RoleWithPermissionsDto {
        role: role.into(),
        permissions,
    }))
}
