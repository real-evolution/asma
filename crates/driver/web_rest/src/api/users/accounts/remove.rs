use axum::extract::Path;
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;
use kernel_repositories::auth::AccountsRepo;

use crate::{error::ApiResult, extractors::di::Dep, util::claims::Claims};

pub async fn remove(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    accounts_repo: Dep<dyn AccountsRepo>,
) -> ApiResult<()> {
    claims.of_with(
        &user_id,
        &[
            (Resource::Users, Action::Modify),
            (Resource::Accounts, Action::Remove),
        ],
    )?;

    Ok(accounts_repo.remove_of(&user_id, &account_id).await?)
}
