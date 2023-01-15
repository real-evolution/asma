use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use itertools::Itertools;
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};

use super::dtos::UserDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    pagination: Pagination,
    state: State<AppState>,
) -> ApiResult<Json<Vec<UserDto>>> {
    auth.in_role(KnownRoles::Admin)?
        .can(&[(Resource::Users, Action::View)])?;

    let users = state
        .data
        .auth()
        .users()
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|user| UserDto { user })
        .collect_vec();

    Ok(Json(users))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
) -> ApiResult<Json<UserDto>> {
    auth.can(&[(Resource::Users, Action::View)])?;

    let user = state.data.auth().users().get(&user_id).await?;

    Ok(Json(UserDto { user }))
}
