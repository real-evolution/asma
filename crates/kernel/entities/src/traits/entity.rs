use chrono::{DateTime, Utc};

pub trait Identifiable {
    type Key;

    fn get_id(&self) -> Self::Key;
}

pub trait Entity: Identifiable {
    fn get_created(&self) -> DateTime<Utc>;
}

pub trait MutableEntity: Entity {
    fn get_updated(&self) -> DateTime<Utc>;
}
