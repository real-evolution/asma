use axum::extract::{Path, State};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{
        auth::{Action, Resource},
        link::Channel,
    },
    traits::Key,
};

use crate::{error::ApiResult, util::auth::token::RestAuthToken};

pub async fn remove(
    auth: RestAuthToken,
    channel_id: Path<Key<Channel>>,
    state: State<AppState>,
) -> ApiResult<()> {
    let channel = state.data.link().channels().get(&channel_id).await?;

    auth.can(&[(Resource::Channel, Action::Remove)])?
        .of(&channel.user_id)?;

    state.data.link().channels().remove(&channel_id).await?;

    Ok(())
}
