use schemars::JsonSchema;

#[derive(Clone, Debug, JsonSchema)]
pub enum AttachmentKind {
    Document,
    Audio,
    Video,
}

#[derive(Clone, Debug, JsonSchema)]
pub struct Attachment {
    pub kind: AttachmentKind,
    pub label: Option<String>,
    pub uri: String,
}
