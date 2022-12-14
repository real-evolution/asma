use chrono::{DateTime, Utc};

pub(crate) type IncomingHandlerUpdate = HandlerUpdate<IncomingMessageUpdate>;
pub(crate) type OutgoingHandlerUpdate = HandlerUpdate<OutgoingMessageUpdate>;

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

pub(crate) struct IncomingMessageUpdate {
    pub chat_id: String,
    pub by_id: String,
    pub kind: IncomingMessageUpdateKind,
    pub sent_at: DateTime<Utc>,
}

pub(crate) struct OutgoingMessageUpdate {
    pub chat_id: String,
    pub kind: OutgoingMessageUpdateKind,
}

pub(crate) enum HandlerUpdate<Message> {
    Message(Message),
}
