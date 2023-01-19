use axum::extract::{Path, State};
use driver_web_common::{state::AppState, auth::validator::AuthValidator};
use kernel_entities::{
    entities::auth::{Account, Action, Resource, User},
    traits::Key,
};
use kernel_services::auth::AuthService;

use super::dtos::UpdateAccountPasswordDto;
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::auth::token::RestAuthToken,
};

pub async fn update_password(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateAccountPasswordDto>,
) -> ApiResult<()> {
    auth.is(&account_id)?
        .can(&[(Resource::Account, Action::Modify)])?;

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
