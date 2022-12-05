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
