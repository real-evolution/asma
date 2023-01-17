mod bots;
mod chats;
mod messages;

pub use bots::*;
pub use chats::*;
pub use messages::*;

pub trait CommDataStore: Send + Sync {
    fn bots(&self) -> &dyn BotsRepo;
}
