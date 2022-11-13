use chrono::{DateTime, Utc};
use kernel_proc_macros::*;

use crate::{traits::*, entities::auth::UserKey};

#[repr(i32)]
#[derive(Debug, Clone, sqlx::Type)]
pub enum ChannelType {
    Telegram = 0,
    WhatsApp = 1,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Channel {
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
    pub valid_until: Option<DateTime<Utc>>,
    pub user_id: UserKey,
}
