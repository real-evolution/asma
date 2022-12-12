use axum::extract::{Path, State};
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::*;
use kernel_entities::traits::Key;

use crate::{error::ApiResult, util::claims::Claims};

pub async fn remove(
    claims: Claims,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    state: State<AppState>,
) -> ApiResult<()> {
    claims.of(&user_id)?.can(&[
        (Resource::Users, Action::Modify),
        (Resource::Accounts, Action::Remove),
    ])?;

    state
        .data
        .auth()
        .accounts()
        .remove_of(&user_id, &account_id)
        .await?;

    Ok(())
}
