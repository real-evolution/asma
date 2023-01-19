use axum::extract::{Path, State};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_services::auth::AuthService;

use super::dtos::{AccountDto, AddAccountDto};
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::{auth::token::RestAuthToken, response::Created},
};

pub async fn add(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<AddAccountDto>,
) -> ApiResult<Created<Key<Account>, AccountDto>> {
    auth.of(&user_id)?.can(&[
        (Resource::User, Action::Modify),
        (Resource::Account, Action::Add),
    ])?;

    let account: AccountDto = state
        .auth
        .add_account_for(
            user_id.clone(),
            form.account_name,
            form.holder_name,
            form.password,
            form.is_active,
        )
        .await?
        .into();

    Ok(Created(
        format!("/api/auth/users/{}/accounts", user_id.value_ref()),
        account.id.clone(),
        account,
    ))
}
