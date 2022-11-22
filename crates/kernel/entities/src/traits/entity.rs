use chrono::{DateTime, Utc};

pub trait Entity {
    type Key: Copy;
    type KeyInner: Copy;

    fn id(&self) -> Self::Key;
    fn created_at(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: Entity {
    fn updated_at(&self) -> DateTime<Utc>;
}
