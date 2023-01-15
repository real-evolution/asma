use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use driver_web_common::{auth::token::AuthToken, state::AppState};
use kernel_services::auth::{models::DeviceInfo, AuthService};

use super::dtos::{TokenPair, UserCredentials};
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::auth::config::RestAuthTokenConfig,
};

pub async fn signin(
    // TypedHeader(agent): TypedHeader<UserAgent>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    state: State<AppState>,
    config: RestAuthTokenConfig,
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
        AuthToken::new(user, account, session, roles, config.into())
            .encode()?;

    Ok(Json(TokenPair {
        access_token,
        refresh_token,
    }))
}
