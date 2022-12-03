use axum::extract::Path;
use kernel_entities::entities::auth::{Account, Action, Resource};
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
    path = "/api/accounts/{account_id}/password",
    request_body = UpdateAccountPasswordDto,
    responses((status = 200, description = "Account password updated")),
    params(
        (
            "account_id" = Key<Account>,
            Path,
            description = "Id of the account to update its password"
        ),
    )
)]
pub async fn update_password(
    claims: Claims,
    account_id: Path<Key<Account>>,
    ValidatedJson(form): ValidatedJson<UpdateAccountPasswordDto>,
    auth_svc: Dep<dyn AuthService>,
) -> ApiResult<()> {
    claims
        .check()
        .is(&account_id)?
        .can(Resource::Accounts, Action::Modify)?;

    Ok(auth_svc
        .update_password(&account_id, &form.old_password, &form.new_password)
        .await?)
}
