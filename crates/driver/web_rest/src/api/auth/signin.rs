use crate::{error::ApiResult, util::validated_json::ValidatedJson};
use common_validation::username;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use validator::Validate;

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
) -> ApiResult<String> {
    Ok(format!(
        "dev={} user={}@{} pass={}",
        form.device_identifier, form.account_name, form.username, form.password
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
