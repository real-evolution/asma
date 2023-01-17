use kernel_entities::{entities::comm::Bot, traits::Key};
use kernel_repositories::{
    comm::{BotsRepo, InsertBot},
    error::RepoResult,
    traits::*,
};
use ormx::{Delete, Table};
use proc_macros::Repo;

use crate::{database::SqlxPool, util::error::map_sqlx_error};

#[derive(Repo)]
#[repo(
    table = "bots",
    read(entity = "Bot", model = "models::BotModel"),
    insert(entity = "InsertBot", model = "models::InsertBotModel")
)]
pub(crate) struct SqlxBotsRepo(pub SqlxPool);

#[async_trait::async_trait]
impl BotsRepo for SqlxBotsRepo {}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::comm::Bot, traits::KeyType};
    use kernel_repositories::comm::InsertBot;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "bots", id = id, insertable, deletable)]
    pub struct BotModel {
        pub id: KeyType,
        pub name: String,
        #[ormx(set)]
        pub is_active: bool,
        #[ormx(get_many)]
        pub user_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    impl From<InsertBot> for InsertBotModel {
        fn from(val: InsertBot) -> Self {
            Self {
                id: uuid::Uuid::new_v4(),
                name: val.name,
                is_active: val.is_active,
                user_id: val.user_id.value(),
            }
        }
    }

    generate_mapping!(Bot, BotModel, 6);
}
