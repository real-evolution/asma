mod bots;

use kernel_repositories::comm::{BotsRepo, CommDataStore};

use crate::database::SqlxPool;

pub(crate) struct SqlxCommDataStore {
    bots: bots::SqlxBotsRepo,
}

impl SqlxCommDataStore {
    pub(crate) fn new(pool: SqlxPool) -> Self {
        Self {
            bots: bots::SqlxBotsRepo(pool),
        }
    }
}

impl CommDataStore for SqlxCommDataStore {
    fn bots(&self) -> &dyn BotsRepo {
        &self.bots
    }
}
