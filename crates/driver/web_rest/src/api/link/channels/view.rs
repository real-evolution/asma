use axum::extract::State;
use axum::{extract::Path, Json};
use driver_web_common::state::AppState;
use itertools::Itertools;
use kernel_entities::entities::{auth::*, link::Channel};
use kernel_entities::traits::Key;

use super::dtos::ChannelDto;
use crate::{
    api::dtos::pagination::Pagination, error::ApiResult,
    extractors::validated_query::ValidatedQuery, util::claims::Claims,
};

pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    state: State<AppState>,
) -> ApiResult<Json<Vec<ChannelDto>>> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Channels, Action::View)],
    )?;

    let channels = state
        .data
        .link()
        .channels()
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|c| ChannelDto::new(c))
        .collect_vec();

    Ok(Json(channels))
}

pub async fn get_by_id(
    claims: Claims,
    channel_id: Path<Key<Channel>>,
    state: State<AppState>,
) -> ApiResult<Json<ChannelDto>> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Channels, Action::View)],
    )?;

    let channel = state.data.link().channels().get(&channel_id).await?;

    Ok(Json(ChannelDto::new(channel)))
}
