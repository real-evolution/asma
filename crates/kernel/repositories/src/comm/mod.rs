mod bots;
mod chats;
mod menus;
mod messages;

pub use bots::*;
pub use chats::*;
pub use menus::*;
pub use messages::*;

pub trait CommDataStore: Send + Sync {
    fn bots(&self) -> &dyn BotsRepo;
    fn menus(&self) -> &dyn MenusRepo;
}
