use axum::{headers::UserAgent, Extension, Json, TypedHeader};
use axum_client_ip::ClientIp;
use kernel_services::auth::{models::DeviceInfo, AuthService};
use utoipa::ToSchema;

use crate::api::dtos::auth::{TokenPair, UserCredentials};
use crate::config::ApiConfig;
use crate::error::ApiResult;
use crate::extractors::di::Dep;
use crate::extractors::validated_json::ValidatedJson;
use crate::util::claims::Claims;

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
    Extension(config): Extension<ApiConfig>,
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
    let jwt = Claims::new(&session, access_rules, &config)
        .encode(&config.token.signing_key.as_bytes())?;

    Ok(Json(TokenPair {
        access_token: jwt,
        refresh_token: session.refresh_token,
    }))
}
