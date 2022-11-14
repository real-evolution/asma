use super::*;

use crate::traits::*;
use kernel_proc_macros::*;

use chrono::{DateTime, Utc};

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub device_identifier: String,
    pub agent: String,
    pub refresh_token: String,
    pub last_address: Option<String>,
    pub account_id: AccountKey,
    pub expires_at: Option<DateTime<Utc>>,
}
