use axum::extract::{Path, State};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};

use super::dtos::UpdateUserDto;
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::auth::token::RestAuthToken,
};

pub async fn update(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateUserDto>,
) -> ApiResult<()> {
    auth.in_role(KnownRoles::Admin)?
        .can(&[(Resource::User, Action::Modify)])?;

    state
        .data
        .auth()
        .users()
        .set_display_name(&user_id, form.display_name)
        .await?;

    Ok(())
}
