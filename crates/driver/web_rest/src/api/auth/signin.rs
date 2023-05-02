use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    headers::UserAgent,
    Json, TypedHeader,
};
use driver_web_common::{
    auth::{token::AuthToken, validator::AuthValidator},
    state::AppState,
};
use kernel_services::auth::{models::DeviceInfo, AuthService};

use super::dtos::{
    TokenPair, TokenRefreshForm, TokenRefreshResponse, UserCredentials,
};
use crate::{
    error::ApiResult,
    extractors::validated_json::ValidatedJson,
    util::auth::{config::RestAuthTokenConfig, token::RestAuthToken},
};

pub async fn signin(
    state: State<AppState>,
    config: RestAuthTokenConfig,
    TypedHeader(agent): TypedHeader<UserAgent>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
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

pub async fn refresh(
    auth: RestAuthToken<true>,
    state: State<AppState>,
    config: RestAuthTokenConfig,
    TypedHeader(agent): TypedHeader<UserAgent>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    Json(form): Json<TokenRefreshForm>,
) -> ApiResult<Json<TokenRefreshResponse>> {
    info!(
        "user `{}@{}` refreshing access token from `{ip}` with agent `{agent}`",
        auth.username, auth.account_name,
    );

    let device_info = DeviceInfo {
        device_identifier: form.device_identifier,
        agent: agent.to_string(),
        last_address: ip.to_string(),
    };

    let session = state
        .auth
        .refresh_session(&form.refresh_token, device_info)
        .await?;

    let account = state
        .data
        .auth()
        .accounts()
        .get(&session.account_id)
        .await?;

    let user = state.data.auth().users().get(&account.user_id).await?;

    auth.of(&user.id)?.is(&account.id)?;

    let roles = state
        .data
        .auth()
        .roles()
        .get_roles_with_permissions_for(&session.account_id)
        .await?;

    let access_token =
        AuthToken::new(user, account, session, roles, config.into());

    Ok(Json(TokenRefreshResponse {
        access_token: access_token.encode()?,
    }))
}
