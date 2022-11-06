use common_validation::username;
use driver_web_common::di::util::axum::Dep;
use kernel_services::config::ConfigService;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{error::ApiResult, util::validated_json::ValidatedJson};

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
    ValidatedJson(form): ValidatedJson<UserCredentials>,
    config_svc: Dep<dyn ConfigService>,
) -> ApiResult<String> {
    Ok(format!(
        "dev={} user={}@{} pass={} | data host: {}",
        form.device_identifier,
        form.account_name,
        form.username,
        form.password,
        config_svc.get_string("xdata.host")?
    ))
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
