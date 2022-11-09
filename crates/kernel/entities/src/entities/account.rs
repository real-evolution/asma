use chrono::{DateTime, Utc};
use kernel_proc_macros::*;

use super::*;
use crate::traits::*;

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Account {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password_hash: String,
    pub is_active: bool,
    pub valid_until: Option<DateTime<Utc>>,
    pub user_id: UserKey,
}

#[derive(Debug, Clone, sqlx::types::Type)]
pub enum OAuth2Provider {
    Google,
    Facebook,
    GitHub,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OAuth2Login {
    pub provider: OAuth2Provider,
    pub provided_name: Option<String>,
    pub provided_identifier: String,
    pub access_token: String,
    pub refresh_token: String,
    pub valid_until: DateTime<Utc>,
    pub account_id: AccountKey,
}
