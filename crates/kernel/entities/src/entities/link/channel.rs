use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;

use crate::{entities::auth::User, traits::*};

#[EnumRepr(type = "i32")]
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub enum ChannelPlatform {
    Telegram = 0,
    WhatsApp = 1,
}

#[entity]
#[derive(Clone, Debug, From, Into, sqlx::FromRow)]
pub struct Channel {
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub user_id: Key<User>,
}
