mod channels;

pub use channels::*;

pub trait LinkDataStore: Send + Sync {
    fn channels(&self) -> &dyn ChannelsRepo;
}
