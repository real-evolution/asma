use axum::{headers::UserAgent, Extension, Json, TypedHeader};
use axum_client_ip::ClientIp;
use common_validation::username;
use kernel_services::auth::{models::DeviceInfo, AuthService};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::config::ApiTokenConfig;
use crate::{
    error::ApiResult,
    extractors::di::Dep,
    util::{jwt::Claims, validated_json::ValidatedJson},
};

#[utoipa::path(
    post,
    path = "/api/auth/signin",
    request_body = UserCredentials,
    responses(
        (status = 200, description = "Signed in successfully", body = TokenPair),
        (status = 403, description = "Invalid credentials were received"),
    ),
)]
pub async fn signin(
    TypedHeader(agent): TypedHeader<UserAgent>,
    ClientIp(ip): ClientIp,
    ValidatedJson(form): ValidatedJson<UserCredentials>,
    Extension(config): Extension<ApiTokenConfig>,
    auth_svc: Dep<dyn AuthService>,
) -> ApiResult<Json<TokenPair>> {
    let device_info = DeviceInfo {
        device_identifier: form.device_identifier,
        agent: agent.to_string(),
        last_address: ip.to_string(),
    };

    let session = auth_svc
        .signin(
            &form.account_name,
            &form.username,
            &form.password,
            device_info,
        )
        .await?;

    let access_rules =
        auth_svc.get_access_rules_for(&session.account_id).await?;
    let jwt = Claims::new(&session, access_rules, &config).to_jwt(&config)?;

    Ok(Json(TokenPair {
        access_token: jwt,
        refresh_token: session.refresh_token,
    }))
}

#[derive(ToSchema, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    #[validate(custom = "username")]
    account_name: String,
    #[validate(custom = "username")]
    username: String,
    device_identifier: String,
    password: String,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    access_token: String,
    refresh_token: String,
}
