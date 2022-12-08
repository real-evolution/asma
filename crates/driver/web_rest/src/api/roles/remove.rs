use axum::{extract::*, Json};
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;

use crate::{error::ApiResult, util::claims::Claims};

use super::dtos::RemoveAccountFromRoleDto;

pub async fn remove(
    claims: Claims,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
) -> ApiResult<()> {
    claims
        .in_role(KnownRoles::Admin)?
        .can(&[(Resource::Roles, Action::Remove)])?;

    state.data.auth().roles().remove(&role_id).await?;

    Ok(())
}

pub async fn remove_permission(
    claims: Claims,
    role_id: Path<Key<Role>>,
    permission_id: Path<Key<Permission>>,
    state: State<AppState>,
) -> ApiResult<()> {
    claims.in_role(KnownRoles::Admin)?.can(&[
        (Resource::Roles, Action::Modify),
        (Resource::Permissions, Action::Remove),
    ])?;

    state
        .data
        .auth()
        .roles()
        .remove_permission(&role_id, &permission_id)
        .await?;

    Ok(())
}

pub async fn remove_from(
    claims: Claims,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    Json(form): Json<RemoveAccountFromRoleDto>,
) -> ApiResult<()> {
    claims
        .in_role(KnownRoles::UserOwner)?
        .can(&[(Resource::Roles, Action::Modify)])?;

    let role = state.data.auth().roles().get(&role_id).await?;
    claims.in_role(role.code.as_str())?;

    let account = state.data.auth().accounts().get(&form.account_id).await?;
    claims.of(&account.user_id)?;

    state
        .data
        .auth()
        .roles()
        .remove_from(&account.id, &role.id)
        .await?;

    Ok(())
}
