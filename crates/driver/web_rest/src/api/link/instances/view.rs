use axum::{
    extract::{Path, Query, State},
    Json,
};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{auth::*, link::Instance},
    traits::Key,
};

use super::dtos::InstanceDto;
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
) -> ApiResult<Json<Vec<InstanceDto>>> {
    auth.can(&[(Resource::Instance, Action::View)])?;

    let instances = match user_id {
        | Some(user_id) => {
            auth.of(&user_id)?;

            state
                .data
                .link()
                .instances()
                .get_by_user_paginated(
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
                .instances()
                .get_paginated(&pagination.before, pagination.page_size)
                .await?
        }
    };

    Ok(Json(instances.into_iter().map(|c| c.into()).collect()))
}

pub async fn get_by_id(
    auth: RestAuthToken,
    instance_id: Path<Key<Instance>>,
    user_id: Option<Query<Key<User>>>,
    state: State<AppState>,
) -> ApiResult<Json<InstanceDto>> {
    auth.can(&[(Resource::Channel, Action::View)])?;

    let instance = match user_id {
        | Some(user_id) => {
            auth.of(&user_id)?;

            state
                .data
                .link()
                .instances()
                .get_of_user(&user_id, &instance_id)
                .await?
        }

        | None => {
            auth.in_role(KnownRoles::Admin)?;

            state.data.link().instances().get(&instance_id).await?
        }
    };

    Ok(Json(instance.into()))
}
