mod bots;
mod menus;

use kernel_repositories::comm::{BotsRepo, CommDataStore, MenusRepo};

use crate::database::SqlxPool;

pub(crate) struct SqlxCommDataStore {
    bots: bots::SqlxBotsRepo,
    menus: menus::SqlxMenusRepo,
}

impl SqlxCommDataStore {
    pub(crate) fn new(pool: SqlxPool) -> Self {
        Self {
            bots: bots::SqlxBotsRepo(pool.clone()),
            menus: menus::SqlxMenusRepo(pool),
        }
    }
}

impl CommDataStore for SqlxCommDataStore {
    fn bots(&self) -> &dyn BotsRepo {
        &self.bots
    }

    fn menus(&self) -> &dyn MenusRepo {
        &self.menus
    }
}
