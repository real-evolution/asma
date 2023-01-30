use axum::extract::{Path, State};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{
        auth::{Action, Resource},
        link::Channel,
    },
    traits::Key,
};
use kernel_repositories::link::UpdateChannel;

use super::dtos::UpdateChannelDto;
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::auth::token::RestAuthToken,
};

pub async fn update(
    auth: RestAuthToken,
    channel_id: Path<Key<Channel>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateChannelDto>,
) -> ApiResult<()> {
    let channel = state.data.link().channels().get(&channel_id).await?;

    auth.can(&[(Resource::Channel, Action::Modify)])?
        .of(&channel.user_id)?;

    state
        .data
        .link()
        .channels()
        .update(
            &channel_id,
            UpdateChannel {
                name: form.name,
                api_key: form.api_key,
                valid_until: form.valid_until,
                is_active: form.is_active,
            },
        )
        .await?;

    Ok(())
}
