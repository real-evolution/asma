use crate::{error::ApiResult, util::validated_json::ValidatedJson};
use common_validation::username;
use serde::Deserialize;

use utoipa::ToSchema;
use validator::Validate;

pub async fn signin(
    ValidatedJson(form): ValidatedJson<UserCredentials>,
) -> ApiResult<String> {
    Ok(format!(
        "dev={} user={}@{} pass={}",
        form.device_identifier, form.account_name, form.username, form.password
    ))
}

#[derive(ToSchema, Validate, Deserialize)]
pub struct UserCredentials {
    #[validate(custom = "username")]
    account_name: String,
    #[validate(custom = "username")]
    username: String,
    device_identifier: String,
    password: String,
}
