mod channels;
mod instances;

pub use channels::*;
pub use instances::*;

pub trait LinkDataStore: Send + Sync {
    fn channels(&self) -> &dyn ChannelsRepo;
    fn instances(&self) -> &dyn InstancesRepo;
}
