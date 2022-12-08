use axum::extract::{Path, State};
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, User};
use kernel_entities::traits::Key;

use super::dtos::UpdateUserDto;
use crate::{
    error::ApiResult, extractors::validated_json::ValidatedJson,
    util::claims::Claims,
};

pub async fn update(
    claims: Claims,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateUserDto>,
) -> ApiResult<()> {
    claims
        .in_role(KnownRoles::Admin)?
        .can(&[(Resource::Users, Action::Modify)])?;

    state
        .data
        .auth()
        .users()
        .set_display_name(&user_id, form.display_name)
        .await?;

    Ok(())
}
