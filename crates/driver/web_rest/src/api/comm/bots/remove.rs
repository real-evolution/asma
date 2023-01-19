use axum::extract::*;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{auth::*, comm::Bot},
    traits::Key,
};

use crate::{error::ApiResult, util::auth::token::RestAuthToken};

pub async fn remove(
    auth: RestAuthToken,
    bot_id: Path<Key<Bot>>,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<()> {
    auth.can(&[(Resource::Bot, Action::Remove)])?;

    match user_id {
        | Some(user_id) => {
            auth.of(&user_id)
                .or_else(|_| auth.in_role(KnownRoles::Admin))?;

            state.data.comm().bots().remove_of(&user_id, &bot_id).await
        }
        | None => {
            auth.in_role(KnownRoles::Admin)?;

            state.data.comm().bots().remove(&bot_id).await
        }
    }?;

    Ok(())
}
