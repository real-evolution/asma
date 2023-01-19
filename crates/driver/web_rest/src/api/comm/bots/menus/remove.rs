use axum::extract::*;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{
        auth::*,
        comm::{Bot, Menu},
    },
    traits::Key,
};

use crate::{error::ApiResult, util::auth::token::RestAuthToken};

pub async fn remove(
    auth: RestAuthToken,
    bot_id: Path<Key<Bot>>,
    menu_id: Path<Key<Menu>>,
    state: State<AppState>,
) -> ApiResult<()> {
    auth.can(&[(Resource::Menu, Action::Remove)])?;

    let menu_bot = state.data.comm().bots().get(&bot_id).await?;

    auth.of(&menu_bot.user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

    state
        .data
        .comm()
        .menus()
        .remove_of(&bot_id, &menu_id)
        .await?;

    Ok(())
}
