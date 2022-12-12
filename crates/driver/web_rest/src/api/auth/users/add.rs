use axum::extract::State;
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::InsertUser;

use super::dtos::AddUserDto;
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::{
        claims::Claims,
        response::{Created, EntityCreated},
    },
};

pub async fn add(
    claims: Claims,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<AddUserDto>,
) -> ApiResult<EntityCreated<User>> {
    claims
        .in_role(KnownRoles::Admin)?
        .can(&[(Resource::Users, Action::Add)])?;

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
