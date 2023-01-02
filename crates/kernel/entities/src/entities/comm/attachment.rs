use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum AttachmentKind {
    Document,
    Audio,
    Video,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Attachment {
    pub kind: AttachmentKind,
    pub label: Option<String>,
    pub uri: String,
}
