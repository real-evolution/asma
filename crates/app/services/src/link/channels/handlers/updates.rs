use chrono::{DateTime, Utc};

pub(crate) type IncomingHandlerUpdate = HandlerUpdate<IncomingMessageUpdate>;
pub(crate) type OutgoingHandlerUpdate = HandlerUpdate<OutgoingMessageUpdate>;

#[derive(Debug)]
pub(crate) enum IncomingMessageUpdateKind {
    New {
        message_id: String,
        content: Option<String>,
    },

    Edit {
        message_id: String,
        content: Option<String>,
    },
}

#[derive(Debug)]
pub(crate) enum OutgoingMessageUpdateKind {
    New {
        content: String,
    },

    Edit {
        message_id: String,
        content: Option<String>,
    },

    Delete {
        message_id: String,
    },
}

#[derive(Debug)]
pub(crate) struct IncomingMessageUpdate {
    pub chat_id: String,
    pub by_id: String,
    pub kind: IncomingMessageUpdateKind,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug)]
pub(crate) struct OutgoingMessageUpdate {
    pub chat_id: String,
    pub kind: OutgoingMessageUpdateKind,
}

#[derive(Debug)]
pub(crate) enum HandlerUpdate<Message> {
    Message(Message),
}
