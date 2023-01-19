use axum::{extract::*, Json};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{entities::auth::*, traits::Key};

use super::dtos::RemoveAccountFromRoleDto;
use crate::{error::ApiResult, util::auth::token::RestAuthToken};

pub async fn remove(
    auth: RestAuthToken,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
) -> ApiResult<()> {
    auth.in_role(KnownRoles::Admin)?
        .can(&[(Resource::Role, Action::Remove)])?;

    state.data.auth().roles().remove(&role_id).await?;

    Ok(())
}

pub async fn remove_permission(
    auth: RestAuthToken,
    role_id: Path<Key<Role>>,
    permission_id: Path<Key<Permission>>,
    state: State<AppState>,
) -> ApiResult<()> {
    auth.in_role(KnownRoles::Admin)?.can(&[
        (Resource::Role, Action::Modify),
        (Resource::Permission, Action::Remove),
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
    auth: RestAuthToken,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    Json(form): Json<RemoveAccountFromRoleDto>,
) -> ApiResult<()> {
    auth.in_role(KnownRoles::UserOwner)?
        .can(&[(Resource::Role, Action::Modify)])?;

    let role = state.data.auth().roles().get(&role_id).await?;
    auth.in_role(role.code.as_str())?;

    let account = state.data.auth().accounts().get(&form.account_id).await?;
    auth.of(&account.user_id)?;

    state
        .data
        .auth()
        .roles()
        .remove_from(&account.id, &role.id)
        .await?;

    Ok(())
}
