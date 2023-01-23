use axum::{
    extract::{Path, Query, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{auth::*, link::Channel},
    traits::Key,
};

use super::dtos::ChannelDto;
use crate::{
    error::ApiResult,
    extractors::pagination::QueryPagination,
    util::auth::token::RestAuthToken,
};

pub async fn get_all(
    auth: RestAuthToken,
    pagination: QueryPagination,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<Json<Vec<ChannelDto>>> {
    auth.can(&[(Resource::Channel, Action::View)])?;

    let channels = match user_id {
        | Some(user_id) => {
            auth.of(&user_id)?;

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
            auth.in_role(KnownRoles::Admin)?;

            state
                .data
                .link()
                .channels()
                .get_paginated(&pagination.before, pagination.page_size)
                .await?
        }
    };

    Ok(Json(channels.into_iter().map(|c| c.into()).collect()))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    channel_id: Path<Key<Channel>>,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<Json<ChannelDto>> {
    auth.can(&[(Resource::Channel, Action::View)])?;

    let channel = match user_id {
        | Some(user_id) => {
            auth.of(&user_id)?;

            state
                .data
                .link()
                .channels()
                .get_of(&user_id, &channel_id)
                .await?
        }

        | None => {
            auth.in_role(KnownRoles::Admin)?;

            state.data.link().channels().get(&channel_id).await?
        }
    };

    Ok(Json(channel.into()))
}
