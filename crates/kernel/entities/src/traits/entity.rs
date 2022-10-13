use chrono::{DateTime, Utc};

pub trait BasicEntity {
    type Key;

    fn get_id(&self) -> Self::Key;
}

pub trait ImmutableEntity: BasicEntity {
    fn get_created(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: ImmutableEntity {
    fn get_updated(&self) -> DateTime<Utc>;
}
