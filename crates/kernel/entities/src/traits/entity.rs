use chrono::{DateTime, Utc};

pub trait Entity {
    type Key: Copy;
    type KeyInner: Copy;

    fn get_id(&self) -> Self::Key;
    fn get_created(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: Entity {
    fn get_updated(&self) -> DateTime<Utc>;
}
