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
    state: State<AppState>,
) -> ApiResult<()> {
    auth.can(&[(Resource::Bots, Action::Remove)])?;

    let bot = state.data.comm().bots().get(&bot_id).await?;

    auth.of(&bot.user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

    state.data.comm().bots().remove(&bot_id).await?;

    Ok(())
}
