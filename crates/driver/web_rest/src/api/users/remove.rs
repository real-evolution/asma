use axum::extract::{Path, State};
use driver_web_common::state::AppState;
use kernel_entities::entities::auth::{Action, KnownRoles, Resource, User};
use kernel_entities::traits::Key;

use crate::{error::ApiResult, util::claims::Claims};

pub async fn remove(
    claims: Claims,
    user_id: Path<Key<User>>,
    state: State<AppState>,
) -> ApiResult<()> {
    claims
        .in_role(KnownRoles::Admin)?
        .can(&[(Resource::Users, Action::Remove)])?;

    state.data.auth().users().remove(&user_id).await?;

    Ok(())
}
