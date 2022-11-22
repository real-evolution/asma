use chrono::{DateTime, Utc};

pub trait BasicEntity {
    type Key: Copy;
    type KeyInner: Copy;

    fn get_id(&self) -> Self::Key;
    fn get_created(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: BasicEntity {
    fn get_updated(&self) -> DateTime<Utc>;
}
