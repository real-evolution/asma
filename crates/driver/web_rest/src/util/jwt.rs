use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: String,
    pub user: Uuid,
    pub account: Uuid,
    pub roles: String,
}
