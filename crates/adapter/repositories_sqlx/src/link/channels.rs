use std::sync::Arc;

use kernel_entities::{entities::link::*, traits::Key};
use kernel_repositories::{
    error::RepoResult,
    link::{ChannelsRepo, InsertChannel},
};
use ormx::Table;
use shaku::Component;

use crate::{database::SqlxDatabaseConnection, util::error::map_sqlx_error};

#[derive(Component)]
#[shaku(interface = ChannelsRepo)]
pub struct SqlxChannelsRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl ChannelsRepo for SqlxChannelsRepo {
    async fn get_by_id(&self, id: &Key<Channel>) -> RepoResult<Channel> {
        Ok(models::ChannelModel::get(self.db.get(), id.value())
            .await
            .map_err(map_sqlx_error)?
            .into())
    }

    async fn create(&self, insert: InsertChannel) -> RepoResult<Key<Channel>> {
        Ok(models::ChannelModel::insert(
            self.db.acquire().await?.as_mut(),
            models::InsertChannelModel {
                id: uuid::Uuid::new_v4(),
                name: insert.name,
                platform: insert.platform,
                api_key: insert.api_key,
                is_active: insert.is_active,
                valid_until: insert.valid_until,
                user_id: insert.user_id.into(),
            },
        )
        .await
        .map_err(map_sqlx_error)?
        .id
        .into())
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::entities::link::{Channel, ChannelPlatform};
    use ormx::Table;
    use uuid::Uuid;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, Table)]
    #[ormx(table = "channels", id = id, insertable, deletable)]
    pub struct ChannelModel {
        pub id: Uuid,
        pub name: String,
        #[ormx(custom_type)]
        pub platform: ChannelPlatform,
        pub api_key: String,
        pub valid_until: Option<DateTime<Utc>>,
        pub is_active: bool,
        #[ormx(custom_type)]
        pub user_id: Uuid,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    generate_mapping!(Channel, ChannelModel, 9);
}
