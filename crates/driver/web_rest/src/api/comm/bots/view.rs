use axum::{
    extract::{Path, State},
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
    auth.can(&[(Resource::Bots, Action::View)])?;

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
    claims: RestAuthToken,
    bot_id: Path<Key<Bot>>,
    state: State<AppState>,
) -> ApiResult<Json<BotDto>> {
    claims.can(&[
        (Resource::Users, Action::View),
        (Resource::Accounts, Action::View),
    ])?;

    let bot = state.data.comm().bots().get(&bot_id).await?;

    claims
        .of(&bot.user_id)
        .or_else(|_| claims.in_role(KnownRoles::Admin))?;

    Ok(Json(bot.into()))
}
