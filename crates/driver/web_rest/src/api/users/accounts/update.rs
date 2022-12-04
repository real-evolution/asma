use axum::extract::Path;
use kernel_entities::entities::auth::{Account, Action, Resource, User};
use kernel_entities::traits::Key;
use kernel_services::auth::AuthService;

use super::dtos::UpdateAccountPasswordDto;
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::claims::Claims,
};

#[utoipa::path(
    put,
    path = "/api/users/{user_id}/accounts/{account_id}/password",
    request_body = UpdateAccountPasswordDto,
    responses((status = 200, description = "Account password updated")),
    responses((status = 401, description = "Invalid old password")),
    responses((status = 404, description = "User or account not found")),
    params(
        (
            "user_id" = Key<User>,
            Path,
            description = "Id of the user to update its account password"
        ),

        (
            "account_id" = Key<Account>,
            Path,
            description = "Id of the account to update its password"
        ),
    )
)]
pub async fn update_password(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    ValidatedJson(form): ValidatedJson<UpdateAccountPasswordDto>,
    auth_svc: Dep<dyn AuthService>,
) -> ApiResult<()> {
    claims.is_with(&account_id, &[(Resource::Accounts, Action::Modify)])?;

    Ok(auth_svc
        .update_password_for(
            &user_id,
            &account_id,
            &form.old_password,
            &form.new_password,
        )
        .await?)
}
