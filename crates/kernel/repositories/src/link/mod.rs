mod channels;
mod instances;
mod peers;

pub use channels::*;
pub use instances::*;
pub use peers::*;

pub trait LinkDataStore: Send + Sync {
    fn channels(&self) -> &dyn ChannelsRepo;
    fn peers(&self) -> &dyn PeersRepo;
    fn instances(&self) -> &dyn InstancesRepo;
}
