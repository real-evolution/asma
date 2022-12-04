use axum::{extract::Path, Json};
use itertools::Itertools;
use kernel_entities::entities::{auth::*, link::Channel};
use kernel_entities::traits::Key;
use kernel_repositories::link::ChannelsRepo;

use super::dtos::ChannelDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    extractors::{di::Dep, validated_query::ValidatedQuery},
    util::claims::Claims,
};

#[utoipa::path(
    get,
    path = "/api/link/channels",
    responses(
        (status = 200, description = "All available channels", body = Vec<ChannelDto>),
    ),
    params(("pagination" = Pagination, Query, description = "Pagination parameters"))
)]
pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    channels_repo: Dep<dyn ChannelsRepo>,
) -> ApiResult<Json<Vec<ChannelDto>>> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Channels, Action::View)],
    )?;

    let channels = channels_repo
        .get_paginated(&pagination.before, pagination.page_size)
        .await?
        .into_iter()
        .map(|c| ChannelDto::new(c))
        .collect_vec();

    Ok(Json(channels))
}

#[utoipa::path(
    get,
    path = "/api/link/channels/{channel_id}",
    responses(
        (status = 200, description = "Channel with `id", body = ChannelDto),
        (status = 404, description = "No channels with `id` were found"),
    ),
    params(
        ("channel_id" = Key<Channel>, Path, description = "Id of the channel to get"),
    )
)]
pub async fn get_by_id(
    claims: Claims,
    channel_id: Path<Key<Channel>>,
    channels_repo: Dep<dyn ChannelsRepo>,
) -> ApiResult<Json<ChannelDto>> {
    claims.in_role_with(
        KnownRoles::Admin,
        &[(Resource::Channels, Action::View)],
    )?;

    Ok(Json(ChannelDto::new(channels_repo.get(&channel_id).await?)))
}
