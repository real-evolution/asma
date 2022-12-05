mod database_connection;
pub mod traits;

pub use database_connection::*;

pub mod auth;
pub mod error;
pub mod link;

pub trait DataStore: Send + Sync {
    fn tx(&self) -> &dyn TransactionManager;
    fn auth(&self) -> &dyn auth::AuthDataStore;
    fn link(&self) -> &dyn link::LinkDataStore;
}
