use axum::{extract::Path, Json};
use kernel_entities::entities::auth::*;
use kernel_repositories::auth::RolesRepo;

use crate::{
    api::dtos::roles::{AddPermissionDto, AddRoleDto},
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{claims::Claims, response::Created},
};

#[utoipa::path(
    post,
    path = "/api/roles",
    responses((status = 201, description = "Role created", body = RoleKey)),
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

    let id = roles_repo.create(form.into()).await?;

    Ok(Created("/api/roles", id).into())
}

#[utoipa::path(
    post,
    path = "/api/roles/{role_id}/permissions",
    responses(
        (
            status = 201,
            description = "Permission created",
            body = PermissionKey,
        )
    ),
)]
pub async fn add_permission(
    claims: Claims,
    Path(id): Path<RoleKey>,
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
        .add_permission(&id, form.resource, form.actions)
        .await?;

    Ok(Created(format!("/api/roles/{id}"), permission_id))
}
