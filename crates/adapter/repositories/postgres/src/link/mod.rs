mod channels;
mod instances;

use kernel_repositories::link::*;

use crate::database::SqlxPool;

pub(crate) struct SqlxLinkDataStore {
    channels: channels::SqlxChannelsRepo,
    instances: instances::SqlxInstancesRepo,
}

impl SqlxLinkDataStore {
    pub(crate) fn new(pool: SqlxPool) -> Self {
        Self {
            channels: channels::SqlxChannelsRepo(pool.clone()),
            instances: instances::SqlxInstancesRepo(pool.clone()),
        }
    }
}

impl LinkDataStore for SqlxLinkDataStore {
    fn channels(&self) -> &dyn ChannelsRepo {
        &self.channels
    }

    fn instances(&self) -> &dyn InstancesRepo {
        &self.instances
    }
}
