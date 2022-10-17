use super::*;

use crate::traits::*;
use kernel_proc_macros::*;

use chrono::{DateTime, Utc};

#[entity]
#[derive(sqlx::FromRow)]
pub struct Session {
    pub device_identifier: String,
    pub agent: String,
    pub last_address: Option<String>,
    pub last_access: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub user_id: UserKey,
    pub account_id: AccountKey,
}
