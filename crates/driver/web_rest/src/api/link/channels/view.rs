use axum::{
    extract::{Path, Query, State},
    Json,
};
use driver_web_common::state::AppState;
use itertools::Itertools;
use kernel_entities::{
    entities::{auth::*, link::Channel},
    traits::Key,
};

use super::dtos::ChannelDto;
use crate::{
    api::dtos::pagination::Pagination,
    error::ApiResult,
    extractors::validated_query::ValidatedQuery,
    util::claims::Claims,
};

pub async fn get_all(
    claims: Claims,
    ValidatedQuery(pagination): ValidatedQuery<Pagination>,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<Json<Vec<ChannelDto>>> {
    claims.can(&[(Resource::Channels, Action::View)])?;

    let channels = match user_id {
        | Some(user_id) => {
            claims.of(&user_id)?;

            state
                .data
                .link()
                .channels()
                .get_paginated_of(
                    &user_id,
                    &pagination.before,
                    pagination.page_size,
                )
                .await?
        }

        | None => {
            claims.in_role(KnownRoles::Admin)?;

            state
                .data
                .link()
                .channels()
                .get_paginated(&pagination.before, pagination.page_size)
                .await?
        }
    };

    Ok(Json(channels.into_iter().map(|c| c.into()).collect_vec()))
}

pub async fn get_by_id(
    claims: Claims,
    channel_id: Path<Key<Channel>>,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<Json<ChannelDto>> {
    claims.can(&[(Resource::Channels, Action::View)])?;

    let channel = match user_id {
        | Some(user_id) => {
            claims.of(&user_id)?;

            state
                .data
                .link()
                .channels()
                .get_of(&user_id, &channel_id)
                .await?
        }

        | None => {
            claims.in_role(KnownRoles::Admin)?;

            state.data.link().channels().get(&channel_id).await?
        }
    };

    Ok(Json(channel.into()))
}
