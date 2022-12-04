use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};
use kernel_repositories::auth::UsersRepo;

use super::dtos::UserDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    extractors::{di::Dep, validated_query::ValidatedQuery},
    util::claims::Claims,
};

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "All available users", body = Vec<UserDto>),
    ),
    params(("pagination" = Pagination, Query, description = "Pagination parameters"))
)]
pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<Json<Vec<UserDto>>> {
    claims
        .in_role_with(KnownRoles::Admin, &[(Resource::Users, Action::View)])?;

    let users = users_repo
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|r| UserDto::new(r))
        .collect_vec();

    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    responses(
        (status = 200, description = "User with `id", body = UserDto),
        (status = 404, description = "No users with `id` were found"),
    ),
    params(
        ("user_id" = Key<User>, Path, description = "Id of the user to get"),
    )
)]
pub async fn get_by_id(
    claims: Claims,
    user_id: Path<Key<User>>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<Json<UserDto>> {
    claims.can(&[(Resource::Users, Action::View)])?;

    Ok(Json(UserDto::new(users_repo.get(&user_id).await?)))
}
