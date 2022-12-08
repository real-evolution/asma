#[derive(Clone, Debug)]
pub enum AttachmentKind {
    Document,
    Audio,
    Video,
}

#[derive(Clone, Debug)]
pub struct Attachment {
    pub kind: AttachmentKind,
    pub label: Option<String>,
    pub uri: String,
}
