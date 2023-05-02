use axum::extract::State;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::entities::{auth::*, link::Channel};
use kernel_repositories::link::InsertChannel;
use kernel_services::link::channels::ChannelsService;

use super::dtos::{AddChannelDto, ChannelDto};
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
) -> ApiResult<EntityCreated<Channel, ChannelDto>> {
    auth.of(&form.user_id)?
        .can(&[(Resource::Channel, Action::Add)])?;

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

    state
        .channels
        .start_channel(&channel.user_id, &channel.id)
        .await
        .unwrap_or_else(|err| {
            warn!(
                "could not start channel #{} of user #{}: {err}",
                channel.id, channel.user_id
            )
        });

    Ok(Created::new("/api/link/channels", channel).into())
}
