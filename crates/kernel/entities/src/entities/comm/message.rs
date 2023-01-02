use chrono::{DateTime, Utc};
use kernel_proc_macros::entity;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::traits::{Entity, Key};

use super::{attachment::Attachment, chat::Chat};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum MessageModificationKind {
    Edit(String),
    Delete,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct MessageModification {
    pub kind: MessageModificationKind,
    pub made_at: DateTime<Utc>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[entity(entity_type = "immutable")]
#[derive(Clone, Debug, JsonSchema)]
pub struct Message {
    pub text: Option<String>,
    pub changes: Option<Vec<MessageModification>>,
    pub attachments: Option<Vec<Attachment>>,
    pub direction: MessageDirection,

    pub delivered_at: Option<DateTime<Utc>>,
    pub seen_at: Option<DateTime<Utc>>,

    pub chat_id: Key<Chat>,
}
