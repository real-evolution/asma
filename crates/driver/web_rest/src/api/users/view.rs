use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::state::AppState;
use itertools::Itertools;
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};

use super::dtos::UserDto;
use crate::{
    api::dtos::pagination::Pagination, error::ApiResult,
    extractors::validated_query::ValidatedQuery, util::claims::Claims,
};

pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    state: State<AppState>,
) -> ApiResult<Json<Vec<UserDto>>> {
    claims
        .in_role_with(KnownRoles::Admin, &[(Resource::Users, Action::View)])?;

    let users = state
        .data
        .auth()
        .users()
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|r| UserDto::new(r))
        .collect_vec();

    Ok(Json(users))
}

pub async fn get_by_id(
    claims: Claims,
    user_id: Path<Key<User>>,
    state: State<AppState>,
) -> ApiResult<Json<UserDto>> {
    claims.can(&[(Resource::Users, Action::View)])?;

    let user = state.data.auth().users().get(&user_id).await?;

    Ok(Json(UserDto::new(user)))
}
