mod channels;
mod instances;
mod peers;

use kernel_repositories::link::*;

use crate::database::SqlxPool;

pub(crate) struct SqlxLinkDataStore {
    channels: channels::SqlxChannelsRepo,
    peers: peers::SqlxPeersRepo,
    instances: instances::SqlxInstancesRepo,
}

impl SqlxLinkDataStore {
    pub(crate) fn new(pool: SqlxPool) -> Self {
        Self {
            channels: channels::SqlxChannelsRepo(pool.clone()),
            peers: peers::SqlxPeersRepo(pool.clone()),
            instances: instances::SqlxInstancesRepo(pool.clone()),
        }
    }
}

impl LinkDataStore for SqlxLinkDataStore {
    fn channels(&self) -> &dyn ChannelsRepo {
        &self.channels
    }

    fn peers(&self) -> &dyn PeersRepo {
        &self.peers
    }

    fn instances(&self) -> &dyn InstancesRepo {
        &self.instances
    }
}
