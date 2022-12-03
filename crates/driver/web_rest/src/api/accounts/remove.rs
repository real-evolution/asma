use axum::extract::Path;
use kernel_entities::entities::auth::{Account, Action, KnownRoles, Resource};
use kernel_entities::traits::Key;
use kernel_repositories::auth::AccountsRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    delete,
    path = "/api/accounts/{account_id}",
    responses((status = 200, description = "Account removed")),
    responses((status = 404, description = "Account not found")),
    params(
        ("account_id" = Key<Account>, Path, description = "Id of the account to remove"),
    )
)]
pub async fn remove(
    claims: Claims,
    account_id: Path<Key<Account>>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<()> {
    claims
        .check()
        .can(Resource::Accounts, Action::Remove)?
        .of(todo!())?;

    Ok(accounts_repo.remove(&account_id).await?)
}
