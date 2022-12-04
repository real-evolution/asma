use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::AccountsRepo;

use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    extractors::{di::Dep, validated_query::ValidatedQuery},
    util::claims::Claims,
};

use super::dtos::AccountDto;

#[utoipa::path(
    get,
    path = "/api/users/{user_id}/accounts",
    responses(
        (
            status = 200,
            description = "All accounts of the specified user",
            body = Vec<AccountDto>
        ),
    ),
    params(
        (
            "user_id" = Key<User>,
            Path,
            description = "Id of the user to get accounts for"
        ),
        ("pagination" = Pagination, Query, description = "Pagination parameters")
    )
)]
pub async fn get_all(
    claims: Claims,
    user_id: Path<Key<User>>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<Json<Vec<AccountDto>>> {
    claims
        .can(&[
            (Resource::Users, Action::View),
            (Resource::Accounts, Action::View),
        ])?
        .of(&user_id)
        .or(claims.in_role(KnownRoles::Admin))?;

    let accounts = accounts_repo
        .get_paginated_of(&user_id, &pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|a| AccountDto::from(a))
        .collect_vec();

    Ok(Json(accounts))
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}/accounts/{account_id}",
    responses(
        (status = 200, description = "Account with `id", body = AccountDto),
        (status = 404, description = "No accounts with `id` were found"),
    ),
    params(
        ("user_id" = Key<User>, Path, description = "Id of the user to get"),
        ("account_id" = Key<Account>, Path, description = "Id of the account to get"),
    )
)]
pub async fn get_by_id(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<Json<AccountDto>> {
    claims
        .can(&[
            (Resource::Users, Action::View),
            (Resource::Accounts, Action::View),
        ])?
        .of(&user_id)
        .or(claims.in_role(KnownRoles::Admin))?;

    let account = accounts_repo.get_of(&user_id, &account_id).await?;

    Ok(Json(AccountDto::from(account)))
}
