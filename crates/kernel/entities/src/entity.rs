use chrono::{DateTime, Utc};

pub trait Entity<Key> {
    fn get_id(&self) -> Key;
    fn get_created(&self) -> DateTime<Utc>;
    fn get_updated(&self) -> DateTime<Utc>;
}
