use axum::{
    extract::{Path, Query, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use itertools::Itertools;
use kernel_entities::{
    entities::{auth::*, comm::Bot},
    traits::Key,
};

use super::dtos::BotDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    state: State<AppState>,
    pagination: Pagination,
) -> ApiResult<Json<Vec<BotDto>>> {
    auth.can(&[(Resource::Bot, Action::View)])?;

    let bots = state
        .data
        .comm()
        .bots()
        .get_paginated_of(
            &auth.user_id,
            &pagination.before,
            pagination.page_size,
        )
        .await?
        .into_iter()
        .map(BotDto::from)
        .collect_vec();

    Ok(Json(bots))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    bot_id: Path<Key<Bot>>,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<Json<BotDto>> {
    auth.can(&[(Resource::Bot, Action::View)])?;

    let bot = match user_id {
        | Some(user_id) => {
            auth.of(&user_id)
                .or_else(|_| auth.in_role(KnownRoles::Admin))?;

            state.data.comm().bots().get_of(&user_id, &bot_id).await
        }
        | None => {
            auth.in_role(KnownRoles::Admin)?;

            state.data.comm().bots().get(&bot_id).await
        }
    }?;

    Ok(Json(bot.into()))
}
