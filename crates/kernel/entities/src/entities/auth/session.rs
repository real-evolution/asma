use super::*;

use crate::traits::*;
use derive_more::{From, Into};
use kernel_proc_macros::*;

use chrono::{DateTime, Utc};

#[entity]
#[derive(Debug, Clone, From, Into, sqlx::FromRow)]
pub struct Session {
    pub device_identifier: String,
    pub agent: String,
    pub refresh_token: String,
    pub last_address: String,
    pub account_id: AccountKey,
    pub expires_at: Option<DateTime<Utc>>,
}
