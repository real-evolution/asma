use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::{headers::UserAgent, Json, TypedHeader};
use driver_web_common::state::AppState;
use kernel_services::auth::models::DeviceInfo;

use super::dtos::{TokenPair, UserCredentials};
use crate::config::ApiConfig;
use crate::error::ApiResult;
use crate::extractors::validated_json::ValidatedJson;
use crate::util::claims::Claims;

pub async fn signin(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    TypedHeader(agent): TypedHeader<UserAgent>,
    state: State<AppState>,
    config: ApiConfig,
    ValidatedJson(form): ValidatedJson<UserCredentials>,
) -> ApiResult<Json<TokenPair>> {
    info!(
        "user `{}@{}` signin-in from `{ip}` with agent `{agent}`",
        form.account_name, form.username
    );

    let device_info = DeviceInfo {
        device_identifier: form.device_identifier,
        agent: agent.to_string(),
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

    let access_rules =
        state.auth.get_access_rules_for(&session.account_id).await?;

    let refresh_token = session.refresh_token.clone();
    let access_token =
        Claims::new(user, account, session, access_rules, config).encode()?;

    Ok(Json(TokenPair {
        access_token,
        refresh_token,
    }))
}
