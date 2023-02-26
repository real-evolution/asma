use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use itertools::Itertools;
use kernel_entities::{entities::auth::*, traits::Key};

use super::dtos::AccountDto;
use crate::{
    error::ApiResult,
    extractors::pagination::QueryPagination,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    pagination: QueryPagination,
) -> ApiResult<Json<Vec<AccountDto>>> {
    auth.can(&[
        (Resource::User, Action::View),
        (Resource::Account, Action::View),
    ])?
    .of(&user_id)
    .or_else(|_| auth.in_role(KnownRoles::Admin))?;

    let accounts = state
        .data
        .auth()
        .accounts()
        .get_paginated_of(&user_id, &pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(AccountDto::from)
        .collect_vec();

    Ok(Json(accounts))
}

pub async fn get_by_id(
    claims: RestAuthToken,
    Path((user_id, account_id)): Path<(Key<User>, Key<Account>)>,
    state: State<AppState>,
) -> ApiResult<Json<AccountDto>> {
    claims
        .can(&[
            (Resource::User, Action::View),
            (Resource::Account, Action::View),
        ])?
        .of(&user_id)
        .or_else(|_| claims.in_role(KnownRoles::Admin))?;

    let account = state
        .data
        .auth()
        .accounts()
        .get_of(&user_id, &account_id)
        .await?;

    Ok(Json(account.into()))
}

pub async fn get_roles_and_permissions(
    auth: RestAuthToken,
    Path((user_id, account_id)): Path<(Key<User>, Key<Account>)>,
    state: State<AppState>,
) -> ApiResult<Json<HashMap<String, Vec<(Resource, Actions)>>>> {
    auth.can(&[(Resource::Role, Action::View)])?
        .of(&user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

    let account = state
        .data
        .auth()
        .accounts()
        .get_of(&user_id, &account_id)
        .await?;

    let roles = state
        .data
        .auth()
        .roles()
        .get_roles_with_permissions_for(&account_id)
        .await?;

    Ok(Json(roles))
}
