use chrono::{DateTime, Utc};
use schemars::JsonSchema;

use crate::traits::{Entity, Key};

use super::{attachment::Attachment, chat::Chat};

#[derive(Clone, Debug, JsonSchema)]
pub enum MessageModificationKind {
    Edit(String),
    Delete,
}

#[derive(Clone, Debug, JsonSchema)]
pub struct MessageModification {
    pub kind: MessageModificationKind,
    pub made_at: DateTime<Utc>,
}

#[derive(Clone, Debug, JsonSchema)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[derive(Clone, Debug, JsonSchema)]
pub struct Message {
    pub id: Key<Message>,

    pub text: Option<String>,
    pub changes: Option<Vec<MessageModification>>,
    pub attachments: Option<Vec<Attachment>>,
    pub direction: MessageDirection,

    pub sent_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub seen_at: Option<DateTime<Utc>>,

    pub chat_id: Key<Chat>,
}

impl Entity for Message {
    fn id(&self) -> &Key<Self> {
        &self.id
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.sent_at
    }
}
