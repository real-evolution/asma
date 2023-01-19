use axum::extract::State;
use driver_web_common::{state::AppState, auth::validator::AuthValidator};
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::InsertUser;

use super::dtos::AddUserDto;
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::{
        auth::token::RestAuthToken,
        response::{Created, EntityCreated},
    },
};

pub async fn add(
    auth: RestAuthToken,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<AddUserDto>,
) -> ApiResult<EntityCreated<User>> {
    auth.in_role(KnownRoles::Admin)?
        .can(&[(Resource::User, Action::Add)])?;

    let user = state
        .data
        .auth()
        .users()
        .create(InsertUser::new(
            form.username,
            form.display_name,
            form.is_active,
        ))
        .await?;

    Ok(Created::new("/api/auth/users", user).into())
}
