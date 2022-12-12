use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::Json;
use driver_web_common::state::AppState;
use kernel_services::auth::models::DeviceInfo;

use super::dtos::{TokenPair, UserCredentials};
use crate::config::ApiConfig;
use crate::error::ApiResult;
use crate::extractors::validated_json::ValidatedJson;
use crate::util::claims::Claims;

pub async fn signin(
    // TypedHeader(agent): TypedHeader<UserAgent>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    state: State<AppState>,
    config: ApiConfig,
    ValidatedJson(form): ValidatedJson<UserCredentials>,
) -> ApiResult<Json<TokenPair>> {
    // TODO:
    // Re-enable agent usage

    info!(
        "user `{}@{}` signin-in from `{ip}` with agent `{{agent}}`",
        form.account_name, form.username
    );

    let device_info = DeviceInfo {
        device_identifier: form.device_identifier,
        agent: "<unknown>".to_owned(), // agent.to_string(),
        last_address: ip.to_string(),
    };

    let (user, account, session) = state
        .auth
        .signin(
            &form.account_name,
            &form.username,
            &form.password,
            device_info,
        )
        .await?;

    let roles = state
        .data
        .auth()
        .roles()
        .get_roles_with_permissions_for(&session.account_id)
        .await?;

    let refresh_token = session.refresh_token.clone();
    let access_token =
        Claims::new(user, account, session, roles, config).encode()?;

    Ok(Json(TokenPair {
        access_token,
        refresh_token,
    }))
}
