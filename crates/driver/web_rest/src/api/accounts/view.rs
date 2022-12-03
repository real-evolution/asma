use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::{
    entities::auth::{Account, Action, KnownRoles, Resource},
    traits::Key,
};
use kernel_repositories::auth::AccountsRepo;

use super::dtos::AccountDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    extractors::{di::Dep, validated_query::ValidatedQuery},
    util::claims::Claims,
};

#[utoipa::path(
    get,
    path = "/api/accounts",
    responses(
        (
            status = 200,
            description = "All available accounts ",
            body = Vec<UserDto>
        ),
    ),
    params(("pagination" = Pagination, Query, description = "Pagination parameters"))
)]
pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<Json<Vec<AccountDto>>> {
    claims
        .check()
        .can(Resource::Accounts, Action::View)?
        .of(todo!())
        .or(claims.check().in_role(&KnownRoles::Admin))?;

    let users = accounts_repo
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|r| AccountDto::from(r))
        .collect_vec();

    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/api/accounts/{account_id}",
    responses(
        (status = 200, description = "Account with `id", body = AccountDto),
        (status = 404, description = "No accounts with `id` were found"),
    ),
    params(
        ("account_id" = Key<Account>, Path, description = "Id of the user to get"),
    )
)]
pub async fn get_by_id(
    claims: Claims,
    account_id: Path<Key<Account>>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<Json<AccountDto>> {
    claims.check().can(Resource::Users, Action::View)?;

    Ok(Json(AccountDto::from(
        accounts_repo.get(&account_id).await?,
    )))
}
