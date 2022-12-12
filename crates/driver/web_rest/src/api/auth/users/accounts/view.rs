use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::state::AppState;
use itertools::Itertools;
use kernel_entities::{entities::auth::*, traits::Key};

use super::dtos::AccountDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    util::claims::Claims,
};

pub async fn get_all(
    claims: Claims,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    pagination: Pagination,
) -> ApiResult<Json<Vec<AccountDto>>> {
    claims
        .can(&[
            (Resource::Users, Action::View),
            (Resource::Accounts, Action::View),
        ])?
        .of(&user_id)
        .or(claims.in_role(KnownRoles::Admin))?;

    let accounts = state
        .data
        .auth()
        .accounts()
        .get_paginated_of(&user_id, &pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|a| AccountDto::from(a))
        .collect_vec();

    Ok(Json(accounts))
}

pub async fn get_by_id(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    state: State<AppState>,
) -> ApiResult<Json<AccountDto>> {
    claims
        .can(&[
            (Resource::Users, Action::View),
            (Resource::Accounts, Action::View),
        ])?
        .of(&user_id)
        .or(claims.in_role(KnownRoles::Admin))?;

    let account = state
        .data
        .auth()
        .accounts()
        .get_of(&user_id, &account_id)
        .await?;

    Ok(Json(account.into()))
}
