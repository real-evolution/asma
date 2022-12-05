mod channels;

use kernel_repositories::link::*;

use crate::database::SqlxPool;

pub(crate) struct SqlxLinkDataStore {
    channels: channels::SqlxChannelsRepo,
}

impl SqlxLinkDataStore {
    pub(crate) fn new(pool: SqlxPool) -> Self {
        Self {
            channels: channels::SqlxChannelsRepo(pool.clone()),
        }
    }
}

impl LinkDataStore for SqlxLinkDataStore {
    fn channels(&self) -> &dyn ChannelsRepo {
        &self.channels
    }
}
