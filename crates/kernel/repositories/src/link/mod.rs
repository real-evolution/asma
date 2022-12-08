mod channels;
mod peers;

pub use channels::*;
pub use peers::*;

pub trait LinkDataStore: Send + Sync {
    fn channels(&self) -> &dyn ChannelsRepo;
}
