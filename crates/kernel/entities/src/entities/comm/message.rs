use chrono::{DateTime, Utc};

use crate::traits::{Entity, Key};

use super::{attachment::Attachment, chat::Chat};

#[derive(Clone, Debug)]

pub enum MessageModicationKind {
    Edit(String),
    Delete,
}

#[derive(Clone, Debug)]
pub struct MessageModification {
    pub kind: MessageModicationKind,
    pub made_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: Key<Message>,
    pub chat_id: Key<Chat>,
    pub text: Option<String>,
    pub modifications: Option<Vec<MessageModification>>,
    pub attachments: Option<Vec<Attachment>>,
    pub sent_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub seen_at: Option<DateTime<Utc>>,
}

impl Entity for Message {
    fn id(&self) -> &Key<Self> {
        &self.id
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.sent_at
    }
}
