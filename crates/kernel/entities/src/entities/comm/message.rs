use chrono::{DateTime, Utc};
use kernel_proc_macros::entity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{attachment::Attachment, chat::Chat};
use crate::{
    entities::{auth::User, link::Instance},
    traits::*,
};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[entity(bson_compat = true)]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde_with::serde_as]
pub struct Message {
    pub text: Option<String>,
    pub changes: Vec<String>,
    pub attachments: Vec<Attachment>,
    pub direction: MessageDirection,
    pub user_id: Key<User>,
    pub chat_id: Key<Chat>,
    pub instance_id: Key<Instance>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub delivered_at: DateTime<Utc>,
    #[serde_as(as = "Option<bson::DateTime>")]
    pub seen_at: Option<DateTime<Utc>>,
    #[serde_as(as = "Option<bson::DateTime>")]
    pub deleted_at: Option<DateTime<Utc>>,
}
