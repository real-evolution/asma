use axum::extract::State;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::entities::{auth::*, comm::Menu};
use kernel_repositories::comm::InsertMenu;

use super::dtos::{AddMenuDto, MenuDto};
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::{
        auth::token::RestAuthToken,
        response::{Created, EntityCreated},
    },
};

pub async fn add(
    auth: RestAuthToken,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<AddMenuDto>,
) -> ApiResult<EntityCreated<Menu, MenuDto>> {
    auth.can(&[(Resource::Menu, Action::Add)])?;

    let menu_bot = state.data.comm().bots().get(&form.bot_id).await?;

    auth.of(&menu_bot.user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

    let menu = state
        .data
        .comm()
        .menus()
        .create(InsertMenu::new(
            form.title,
            form.content,
            form.menu_trigger,
            form.matching_strategy,
            form.is_active,
            form.parent_menu_id,
            form.bot_id,
        ))
        .await?;

    Ok(Created::new("/api/comm/bots/menus", menu).into())
}
