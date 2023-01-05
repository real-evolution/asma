use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use enum_repr::EnumRepr;
use kernel_proc_macros::*;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};

use crate::{entities::auth::User, traits::*};

#[EnumRepr(type = "i32")]
#[derive(Clone, Copy, Debug, JsonSchema_repr, Deserialize, Serialize)]
pub enum ChannelPlatform {
    Telegram = 0,
    // WhatsApp = 1,
}

#[entity]
#[derive(Clone, Debug, From, Into, JsonSchema)]
pub struct Channel {
    pub name: String,
    pub platform: ChannelPlatform,
    pub api_key: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub max_instances: Option<i64>,
    pub user_id: Key<User>,
}

impl From<i32> for ChannelPlatform {
    fn from(value: i32) -> Self {
        Self::from_repr(value)
            .expect(&format!("invalid channel platform: {value}"))
    }
}

impl From<ChannelPlatform> for i32 {
    fn from(val: ChannelPlatform) -> Self {
        val.repr()
    }
}
