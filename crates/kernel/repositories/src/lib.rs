mod database_connection;

pub use database_connection::*;

pub mod auth;
pub mod comm;
pub mod error;
pub mod link;
pub mod traits;

pub trait DataStore: Send + Sync {
    fn tx(&self) -> &dyn TransactionManager;
    fn auth(&self) -> &dyn auth::AuthDataStore;
    fn link(&self) -> &dyn link::LinkDataStore;
    fn comm(&self) -> &dyn comm::CommDataStore;
}

pub trait DocumentStore: Send + Sync {
    fn chats(&self) -> &dyn comm::ChatsRepo;
    fn messages(&self) -> &dyn comm::MessagesRepo;
}
