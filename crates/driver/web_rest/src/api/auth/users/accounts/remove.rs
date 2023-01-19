use axum::extract::{Path, State};
use driver_web_common::{state::AppState, auth::validator::AuthValidator};
use kernel_entities::{entities::auth::*, traits::Key};

use crate::{error::ApiResult, util::auth::token::RestAuthToken};

pub async fn remove(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    account_id: Path<Key<Account>>,
    state: State<AppState>,
) -> ApiResult<()> {
    auth.of(&user_id)?.can(&[
        (Resource::User, Action::Modify),
        (Resource::Account, Action::Remove),
    ])?;

    state
        .data
        .auth()
        .accounts()
        .remove_of(&user_id, &account_id)
        .await?;

    Ok(())
}
