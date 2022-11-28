use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};
use kernel_repositories::auth::AccountsRepo;

use crate::{
    api::{accounts::dtos::AccountDto, dtos::pagination::Pagination},
    error::ApiResult,
    extractors::{di::Dep, validated_query::ValidatedQuery},
    util::claims::Claims,
};

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
pub async fn get_accounts_of(
    claims: Claims,
    user_id: Path<Key<User>>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<Json<Vec<AccountDto>>> {
    claims.require_any_role_with_permissions(
        vec![KnownRoles::Root, KnownRoles::Admin],
        vec![
            (Resource::Users, Action::View),
            (Resource::Accounts, Action::View),
        ],
    )?;

    let accounts = accounts_repo
        .get_paginated_for(&user_id, &pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|a| AccountDto::from(a))
        .collect_vec();

    Ok(Json(accounts))
}
