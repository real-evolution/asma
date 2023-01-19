use axum::extract::{Path, State};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::auth::{Action, KnownRoles, Resource, User},
    traits::Key,
};

use crate::{error::ApiResult, util::auth::token::RestAuthToken};

pub async fn remove(
    auth: RestAuthToken,
    user_id: Path<Key<User>>,
    state: State<AppState>,
) -> ApiResult<()> {
    auth.in_role(KnownRoles::Admin)?
        .can(&[(Resource::User, Action::Remove)])?;

    state.data.auth().users().remove(&user_id).await?;

    Ok(())
}
