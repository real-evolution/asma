use kernel_entities::{entities::link::*, traits::Key};
use kernel_repositories::{error::RepoResult, link::*, traits::*};
use ormx::{Delete, Table};
use proc_macros::Repo;

use crate::{database::SqlxPool, util::error::map_sqlx_error};

#[derive(Repo)]
#[repo(
    table = "channels",
    read(entity = "Channel", model = "models::ChannelModel"),
    insert(entity = "InsertChannel", model = "models::InsertChannelModel")
)]
pub(crate) struct SqlxChannelsRepo(pub SqlxPool);

#[async_trait::async_trait]
impl ChannelsRepo for SqlxChannelsRepo {}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::link::Channel, traits::KeyType};
    use kernel_repositories::link::InsertChannel;
    use ormx::Table;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, Table)]
    #[ormx(table = "channels", id = id, insertable, deletable)]
    pub struct ChannelModel {
        pub id: KeyType,
        pub name: String,
        pub platform: i32,
        pub api_key: String,
        pub valid_until: Option<DateTime<Utc>>,
        #[ormx(set)]
        pub is_active: bool,
        #[ormx(default, set)]
        pub max_instances: Option<i64>,
        pub user_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    impl Into<InsertChannelModel> for InsertChannel {
        fn into(self) -> InsertChannelModel {
            InsertChannelModel {
                id: uuid::Uuid::new_v4(),
                user_id: self.user_id.into(),
                name: self.name,
                platform: self.platform.into(),
                api_key: self.api_key,
                valid_until: self.valid_until,
                is_active: self.is_active,
            }
        }
    }

    generate_mapping!(Channel, ChannelModel, 10);
}
