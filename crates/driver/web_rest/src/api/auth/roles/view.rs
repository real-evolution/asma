use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{
    auth::{util::AuthedRepoExts, validator::AuthValidator},
    state::AppState,
};
use itertools::Itertools;
use kernel_entities::{
    entities::auth::{KnownRoles, Role},
    traits::Key,
};

use super::dtos::{PermissionDto, RoleDto, RoleWithPermissionsDto};
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    pagination: Pagination,
    state: State<AppState>,
) -> ApiResult<Json<Vec<RoleDto>>> {
    auth.in_role(KnownRoles::Admin)?;

    let roles = state
        .data
        .auth()
        .roles()
        .get_paginated_authed(&pagination.before, pagination.page_size, &auth)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect_vec();

    Ok(Json(roles))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
) -> ApiResult<Json<RoleWithPermissionsDto>> {
    let role = state
        .data
        .auth()
        .roles()
        .get_authed(&role_id, &auth)
        .await?;

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
