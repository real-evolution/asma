use axum::extract::{Path, State};
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::{
        auth::{Action, Resource},
        link::Instance,
    },
    traits::Key,
};
use kernel_repositories::link::UpdateInstance;

use super::dtos::UpdateInstanceDto;
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::auth::token::RestAuthToken,
};

pub async fn update(
    auth: RestAuthToken,
    instance_id: Path<Key<Instance>>,
    state: State<AppState>,
    ValidatedJson(form): ValidatedJson<UpdateInstanceDto>,
) -> ApiResult<()> {
    auth.can(&[(Resource::Instance, Action::Modify)])?;

    let instance = state
        .data
        .link()
        .instances()
        .get_of_user(&auth.user_id, &instance_id)
        .await?;

    state
        .data
        .link()
        .instances()
        .update(
            &instance.id,
            UpdateInstance {
                display_name: form.display_name,
                phone_number: form.phone_number,
            },
        )
        .await?;

    Ok(())
}
