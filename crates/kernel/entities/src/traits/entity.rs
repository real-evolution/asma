use chrono::{DateTime, Utc};

pub trait BasicEntity {
    type Key;

    fn get_id(&self) -> Self::Key;
    fn get_created(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: BasicEntity {
    fn get_updated(&self) -> DateTime<Utc>;
}
