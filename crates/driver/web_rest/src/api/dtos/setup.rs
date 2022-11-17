use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RootAccountDetails {
    #[validate(length(min = 4))]
    pub holder_name: Option<String>,
    #[validate(length(min = 8))]
    pub password: String,
}
