use chrono::{DateTime, Utc};
use kernel_proc_macros::*;

use crate::{traits::*, entities::auth::UserKey};

#[repr(i32)]
#[derive(Debug, Clone, sqlx::Type)]
pub enum ChannelPlatform {
    Telegram = 0,
    WhatsApp = 1,
}

#[entity]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Channel {
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub user_id: UserKey,
}
