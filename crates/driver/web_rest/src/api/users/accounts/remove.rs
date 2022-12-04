use axum::extract::Path;
use kernel_entities::entities::auth::{Account, Action, KnownRoles, Resource};
use kernel_entities::traits::Key;
use kernel_repositories::auth::AccountsRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}/accounts/{account_id}",
    responses((status = 200, description = "Account removed")),
    responses((status = 404, description = "User or account not found")),
    params(
        ("user_id" = Key<Account>, Path, description = "Id of the user to remove the account of"),
        ("account_id" = Key<Account>, Path, description = "Id of the account to remove"),
    )
)]
pub async fn remove(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<()> {
    claims.of_with(&user_id, &[(Resource::Accounts, Action::Remove)])?;

    Ok(accounts_repo.remove_of(&account_id, &user_id).await?)
}
