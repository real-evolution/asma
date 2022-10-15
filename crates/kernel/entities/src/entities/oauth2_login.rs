use super::*;
use crate::traits::*;

use kernel_proc_macros::*;

use chrono::{DateTime, Utc};

#[derive(sqlx::types::Type)]
pub enum OAuth2Provider {
    Google,
    Facebook,
    GitHub,
}

#[entity]
#[derive(sqlx::FromRow)]
pub struct OAuth2Login {
    pub provider: OAuth2Provider,
    pub provided_name: Option<String>,
    pub provided_identifier: String,
    pub access_token: String,
    pub refresh_token: String,
    pub valid_until: DateTime<Utc>,
    pub account_id: AccountKey,
}
