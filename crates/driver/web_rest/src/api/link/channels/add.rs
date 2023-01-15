use axum::extract::State;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::entities::{auth::*, link::Channel};
use kernel_repositories::link::InsertChannel;

use super::dtos::AddChannelDto;
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
    ValidatedJson(form): ValidatedJson<AddChannelDto>,
) -> ApiResult<EntityCreated<Channel>> {
    auth.of(&form.user_id)?
        .can(&[(Resource::Channels, Action::Add)])?;

    let channel = state
        .data
        .link()
        .channels()
        .create(InsertChannel::new(
            form.user_id,
            form.name,
            form.platform,
            form.api_key,
            form.valid_until,
            form.is_active,
        ))
        .await?;

    Ok(Created::new("/api/link/channels", channel).into())
}
