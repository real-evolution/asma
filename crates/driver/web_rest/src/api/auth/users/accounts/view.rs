use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{state::AppState, auth::validator::AuthValidator};
use itertools::Itertools;
use kernel_entities::{entities::auth::*, traits::Key};

use super::dtos::AccountDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    pagination: Pagination,
) -> ApiResult<Json<Vec<AccountDto>>> {
    auth.can(&[
        (Resource::Users, Action::View),
        (Resource::Accounts, Action::View),
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
            (Resource::Users, Action::View),
            (Resource::Accounts, Action::View),
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
