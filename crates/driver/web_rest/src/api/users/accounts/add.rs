use axum::extract::{Path, State};
use driver_web_common::state::AppState;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::InsertAccount;

use super::dtos::{AccountDto, AddAccountDto};
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::{claims::Claims, response::Created},
};

pub async fn add(
    claims: Claims,
    user_id: Path<Key<User>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<AddAccountDto>,
) -> ApiResult<Created<Key<Account>, AccountDto>> {
    claims.of_with(
        &user_id,
        &[
            (Resource::Users, Action::Modify),
            (Resource::Accounts, Action::Add),
        ],
    )?;

    let account: AccountDto = state
        .data
        .auth()
        .accounts()
        .create(InsertAccount::new(
            user_id.0.clone(),
            form.account_name,
            form.holder_name,
            state.hash.hash(&form.password)?,
            form.is_active.into(),
        ))
        .await?
        .into();

    Ok(Created(
        format!("/api/users/{}/accounts", user_id.value_ref()),
        account.id.clone(),
        account,
    ))
}
