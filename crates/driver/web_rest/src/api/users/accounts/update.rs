use axum::extract::{Path, State};
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::{Account, Action, Resource, User};
use kernel_entities::traits::Key;

use super::dtos::UpdateAccountPasswordDto;
use crate::{
    error::ApiResult, extractors::validated_json::ValidatedJson,
    util::claims::Claims,
};

pub async fn update_password(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateAccountPasswordDto>,
) -> ApiResult<()> {
    claims.is_with(&account_id, &[(Resource::Accounts, Action::Modify)])?;

    state
        .auth
        .update_password_for(
            &user_id,
            &account_id,
            &form.old_password,
            &form.new_password,
        )
        .await?;

    Ok(())
}
