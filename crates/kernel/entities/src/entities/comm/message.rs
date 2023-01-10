use chrono::{DateTime, Utc};
use kernel_proc_macros::entity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{entities::link::Instance, traits::*};

use super::{attachment::Attachment, chat::Chat};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[entity]
#[derive(Clone, Debug, JsonSchema)]
pub struct Message {
    pub text: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub changes: Vec<String>,
    pub direction: MessageDirection,

    pub delivered_at: Option<DateTime<Utc>>,
    pub seen_at: Option<DateTime<Utc>>,

    pub chat_id: Key<Chat>,
}
