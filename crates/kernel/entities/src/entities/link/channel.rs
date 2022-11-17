use chrono::{DateTime, Utc};
use kernel_proc_macros::*;

use crate::{entities::auth::UserKey, traits::*};

#[repr(i32)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
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
