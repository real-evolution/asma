use axum::{
    extract::{Path, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use itertools::Itertools;
use kernel_entities::{
    entities::{
        auth::*,
        comm::{Bot, Menu},
    },
    traits::Key,
};

use super::dtos::MenuDto;
use crate::{
    error::ApiResult,
    util::auth::token::RestAuthToken, extractors::pagination::QueryPagination,
};

pub async fn get_all(
    auth: RestAuthToken,
    state: State<AppState>,
    bot_id: Path<Key<Bot>>,
    pagination: QueryPagination,
) -> ApiResult<Json<Vec<MenuDto>>> {
    auth.can(&[(Resource::Menu, Action::View)])?;

    let menus = state
        .data
        .comm()
        .menus()
        .get_paginated_of(&bot_id, &pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(MenuDto::from)
        .collect_vec();

    Ok(Json(menus))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    bot_id: Path<Key<Bot>>,
    menu_id: Path<Key<Menu>>,
    state: State<AppState>,
) -> ApiResult<Json<MenuDto>> {
    auth.can(&[(Resource::Menu, Action::View)])?;

    Ok(Json(
        state
            .data
            .comm()
            .menus()
            .get_of(&bot_id, &menu_id)
            .await?
            .into(),
    ))
}
