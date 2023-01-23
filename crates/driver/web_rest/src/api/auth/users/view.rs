use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{
    auth::{util::RepoExt, validator::AuthValidator},
    state::AppState,
};
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};

use super::dtos::UserDto;
use crate::{
    error::ApiResult,
    extractors::pagination::QueryPagination,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    pagination: QueryPagination,
    state: State<AppState>,
) -> ApiResult<Json<Vec<UserDto>>> {
    auth.in_role(KnownRoles::Admin)?;

    let users = state
        .data
        .auth()
        .users()
        .get_paginated_authed(&pagination.before, pagination.page_size, &auth)
        .await?
        .into_iter()
        .map(|user| UserDto { user })
        .collect::<Vec<_>>();

    Ok(Json(users))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
) -> ApiResult<Json<UserDto>> {
    auth.can(&[(Resource::User, Action::View)])?;

    let user = state.data.auth().users().get(&user_id).await?;

    Ok(Json(UserDto { user }))
}
