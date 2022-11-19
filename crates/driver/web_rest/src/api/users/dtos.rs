use derive_more::Constructor;
use kernel_entities::entities::auth::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Constructor, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    #[serde(flatten)]
    pub user: User,
}
