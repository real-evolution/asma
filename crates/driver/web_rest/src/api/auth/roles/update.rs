use axum::extract::{Path, State};
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, Role};
use kernel_entities::traits::Key;

use super::dtos::UpdateRoleDto;
use crate::{
    error::ApiResult, extractors::validated_json::ValidatedJson,
    util::claims::Claims,
};

pub async fn update(
    claims: Claims,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateRoleDto>,
) -> ApiResult<()> {
    claims
        .in_role(KnownRoles::Admin)?
        .can(&[(Resource::Roles, Action::Modify)])?;

    state
        .data
        .auth()
        .roles()
        .set_friendly_name(&role_id, form.friendly_name)
        .await?;

    Ok(())
}
