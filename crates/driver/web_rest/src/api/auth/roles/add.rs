use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::InsertRole;

use super::dtos::{AddAccountToRoleDto, AddPermissionDto, AddRoleDto};
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
    ValidatedJson(form): ValidatedJson<AddRoleDto>,
) -> ApiResult<EntityCreated<Role>> {
    auth.in_role(KnownRoles::Admin)?
        .can(&[(Resource::Role, Action::Add)])?;

    let role = state
        .data
        .auth()
        .roles()
        .create(InsertRole::new(form.code, form.friendly_name))
        .await?;

    Ok(Created::new("/api/auth/roles", role).into())
}

pub async fn add_permission(
    auth: RestAuthToken,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    Json(form): Json<AddPermissionDto>,
) -> ApiResult<EntityCreated<Permission>> {
    auth.is_root()?;

    let permission = state
        .data
        .auth()
        .roles()
        .add_permission(&role_id, form.resource, form.actions)
        .await?;

    Ok(Created::new(
        format!("/api/auth/roles/{}", role_id.0),
        permission,
    ))
}

pub async fn add_to(
    auth: RestAuthToken,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    Json(form): Json<AddAccountToRoleDto>,
) -> ApiResult<()> {
    auth.in_role(KnownRoles::UserOwner)?;

    let role = state.data.auth().roles().get(&role_id).await?;
    auth.in_role(role.code.as_str())?;

    let account = state.data.auth().accounts().get(&form.account_id).await?;
    auth.of(&account.user_id)?;

    state
        .data
        .auth()
        .roles()
        .add_to(&account.id, &role.id)
        .await?;

    Ok(())
}
