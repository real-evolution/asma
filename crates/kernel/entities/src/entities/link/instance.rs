use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use kernel_proc_macros::*;
use schemars::JsonSchema;

use super::Channel;
use crate::{entities::comm::Chat, traits::*};

#[entity]
#[derive(Clone, Debug, From, Into, JsonSchema)]
pub struct Instance {
    pub platform_identifier: i64,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub phone_number: Option<String>,
    pub last_active: Option<DateTime<Utc>>,
    pub channel_id: Key<Channel>,
}
