use axum::extract::State;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::entities::{auth::*, comm::Bot};
use kernel_repositories::comm::InsertBot;

use super::dtos::{AddBotDto, BotDto};
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
    ValidatedJson(form): ValidatedJson<AddBotDto>,
) -> ApiResult<EntityCreated<Bot, BotDto>> {
    auth.can(&[(Resource::Bot, Action::Add)])?
        .of(&form.user_id)
        .or_else(|_| auth.in_role(KnownRoles::Admin))?;

    let bot = state
        .data
        .comm()
        .bots()
        .create(InsertBot::new(form.name, form.is_active, form.user_id))
        .await?;

    Ok(Created::new("/api/comm/bots", bot).into())
}
