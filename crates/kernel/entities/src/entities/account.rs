use super::*;
use crate::traits::*;
use kernel_proc_macros::*;

use chrono::{DateTime, Utc};

#[entity]
#[derive(sqlx::FromRow)]
pub struct Account {
    pub account_name: String,
    pub holder_name: Option<String>,
    pub password: Option<String>,
    pub is_active: bool,
    pub valid_until: Option<DateTime<Utc>>,
    pub user_id: UserKey,
}
