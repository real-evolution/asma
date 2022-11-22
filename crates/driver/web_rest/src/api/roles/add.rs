use axum::{extract::Path, Json};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::{InsertRole, RolesRepo};

use super::dtos::{AddPermissionDto, AddRoleDto};
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{claims::Claims, response::Created},
};

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
) -> ApiResult<Created<Key<Role>>> {
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
        (status = 201, description = "Permission created", body = Key<Permission>),
        (status = 404, description = "Role not found"),
    ),
    params(
        (
            "role_id" = Key<Role>,
            Path,
            description = "Id of the role to add the permission to"
        ),
    )
)]
pub async fn add_permission(
    claims: Claims,
    Path(role_id): Path<Key<Role>>,
    Json(form): Json<AddPermissionDto>,
    roles_repo: Dep<dyn RolesRepo>,
) -> ApiResult<Created<Key<Permission>>> {
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
