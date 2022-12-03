use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::auth::{AccountsRepo, InsertAccount};
use kernel_services::crypto::hash::CryptoHashService;

use super::dtos::{AccountDto, AddAccountDto};
use crate::{
    error::ApiResult,
    extractors::{di::Dep, validated_json::ValidatedJson},
    util::{claims::Claims, response::Created},
};

#[utoipa::path(
    post,
    path = "/api/accounts",
    request_body = AddAccountDto,
    responses(
        (status = 201, description = "Account created"),
        (status = 404, description = "User not found"),
    ),
)]
pub async fn add(
    claims: Claims,
    ValidatedJson(form): ValidatedJson<AddAccountDto>,
    accounts_repo: Dep<dyn AccountsRepo>,
    hash_svc: Dep<dyn CryptoHashService>,
) -> ApiResult<Created<Key<Account>, AccountDto>> {
    claims
        .check()
        .can(Resource::Accounts, Action::Add)?
        .in_any(&[KnownRoles::Admin])?
        .of(&form.user_id)?;

    let password_hash = hash_svc.hash(&form.password)?;
    let state = if form.is_active {
        AccountState::Active
    } else {
        AccountState::Inactive
    };

    let account: AccountDto = accounts_repo
        .create(InsertAccount::new(
            form.user_id,
            form.account_name,
            form.holder_name,
            password_hash,
            state,
        ))
        .await?
        .into();

    Ok(Created("/api/accounts".into(), account.id.clone(), account))
}
