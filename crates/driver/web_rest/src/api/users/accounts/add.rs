use axum::extract::Path;
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
    path = "/api/users/{user_id}/accounts",
    request_body = AddAccountDto,
    responses(
        (status = 201, description = "Account created"),
        (status = 404, description = "User not found"),
    ),
    params(
        ("user_id" = Key<Account>, Path, description = "Id of the user to add the account to"),
    )
)]
pub async fn add(
    claims: Claims,
    user_id: Path<Key<User>>,
    ValidatedJson(form): ValidatedJson<AddAccountDto>,
    accounts_repo: Dep<dyn AccountsRepo>,
    hash_svc: Dep<dyn CryptoHashService>,
) -> ApiResult<Created<Key<Account>, AccountDto>> {
    claims.of_with(
        &user_id,
        &[
            (Resource::Users, Action::Modify),
            (Resource::Accounts, Action::Add),
        ],
    )?;

    let account: AccountDto = accounts_repo
        .create(InsertAccount::new(
            user_id.0.clone(),
            form.account_name,
            form.holder_name,
            hash_svc.hash(&form.password)?,
            if form.is_active {
                AccountState::Active
            } else {
                AccountState::Inactive
            },
        ))
        .await?
        .into();

    Ok(Created(
        format!("/api/users/{}/accounts", user_id.value_ref()),
        account.id.clone(),
        account,
    ))
}
