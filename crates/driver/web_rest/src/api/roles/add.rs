use axum::{extract::Path, Json};
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::{InsertRole, RolesRepo};

use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{claims::Claims, response::Created},
};

use super::dtos::{AddRoleDto, AddPermissionDto};

#[utoipa::path(
    post,
    path = "/api/roles",
    request_body = AddRoleDto,
    responses((status = 201, description = "Role created")),
)]
pub async fn add(
    claims: Claims,
    ValidatedJson(form): ValidatedJson<AddRoleDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Created<RoleKey>> {
    claims.require_role_with_permission(
        KnownRoles::Root,
        (Resource::Roles, Action::Add),
    )?;

    let id = roles_repo
        .create(InsertRole::new(form.code, form.friendly_name))
        .await?;

    Ok(Created("/api/roles", id).into())
}

#[utoipa::path(
    post,
    path = "/api/roles/{role_id}/permissions",
    request_body = AddPermissionDto,
    responses(
        (
            status = 201,
            description = "Permission created",
            body = PermissionKey,
        )
    ),
    params(
        (
            "role_id" = RoleKey,
            Path,
            description = "Id of the role to add the permission to"
        ),
    )
)]
pub async fn add_permission(
    claims: Claims,
    Path(role_id): Path<RoleKey>,
    Json(form): Json<AddPermissionDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Created<PermissionKey>> {
    claims.require_role_with_permissions(
        KnownRoles::Root,
        vec![
            (Resource::Roles, Action::Modify),
            (Resource::Permissions, Action::Add),
        ],
    )?;

    let permission_id = roles_repo
        .add_permission(&role_id, form.resource, form.actions)
        .await?;

    Ok(Created(format!("/api/roles/{role_id}"), permission_id))
}
