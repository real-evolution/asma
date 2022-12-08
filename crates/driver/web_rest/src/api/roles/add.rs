use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::state::AppState;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::InsertRole;

use super::dtos::{AddAccountToRoleDto, AddPermissionDto, AddRoleDto};
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
    ValidatedJson(form): ValidatedJson<AddRoleDto>,
) -> ApiResult<EntityCreated<Role>> {
    claims
        .in_role(KnownRoles::Admin)?
        .can(&[(Resource::Roles, Action::Add)])?;

    let role = state
        .data
        .auth()
        .roles()
        .create(InsertRole::new(form.code, form.friendly_name))
        .await?;

    Ok(Created::new("/api/roles", role).into())
}

pub async fn add_permission(
    claims: Claims,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    Json(form): Json<AddPermissionDto>,
) -> ApiResult<EntityCreated<Permission>> {
    claims.is_root()?;

    let permission = state
        .data
        .auth()
        .roles()
        .add_permission(&role_id, form.resource, form.actions)
        .await?;

    Ok(Created::new(
        format!("/api/roles/{}", role_id.0),
        permission,
    ))
}

pub async fn add_to(
    claims: Claims,
    role_id: Path<Key<Role>>,
    state: State<AppState>,
    Json(form): Json<AddAccountToRoleDto>,
) -> ApiResult<()> {
    claims.in_role(KnownRoles::UserOwner)?;

    let role = state.data.auth().roles().get(&role_id).await?;
    claims.in_role(role.code.as_str())?;

    let account = state.data.auth().accounts().get(&form.account_id).await?;
    claims.of(&account.user_id)?;

    state
        .data
        .auth()
        .roles()
        .add_to(&account.id, &role.id)
        .await?;

    Ok(())
}
