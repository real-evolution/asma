use axum::{extract::Path, Json};
use kernel_entities::entities::auth::{Action, Resource, UserKey};
use kernel_repositories::auth::UsersRepo;

use super::dtos::UserDto;
use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    responses(
        (status = 200, description = "User with `id", body = UserDto),
        (status = 404, description = "No users with `id` were found"),
    ),
    params(
        ("user_id" = Userkey, Path, description = "Id of the user to get"),
    )
)]
pub async fn get_by_id(
    claims: Claims,
    Path(user_id): Path<UserKey>,
    users_repo: Dep<dyn UsersRepo>,
) -> ApiResult<Json<UserDto>> {
    claims
        .require_permission(Resource::Users, Action::View | Action::Global)?;

    Ok(Json(UserDto::new(users_repo.get(&user_id).await?)))
}
