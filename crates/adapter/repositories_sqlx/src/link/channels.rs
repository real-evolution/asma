use std::sync::Arc;

use kernel_entities::{entities::link::*, traits::Key};
use kernel_repositories::{
    error::RepoResult,
    link::{ChannelsRepo, InsertChannel},
};
use ormx::Table;
use shaku::Component;

use crate::{
    database::SqlxDatabaseConnection, models::link::channel::ChannelModel,
    util::map_sqlx_error,
};

#[derive(Component)]
#[shaku(interface = ChannelsRepo)]
pub struct SqlxChannelsRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl ChannelsRepo for SqlxChannelsRepo {
    async fn get_by_id(&self, id: &Key<Channel>) -> RepoResult<Channel> {
        Ok(ChannelModel::get(self.db.get(), id.value())
            .await
            .map_err(map_sqlx_error)?
            .into())
    }

    async fn create(&self, insert: InsertChannel) -> RepoResult<Key<Channel>> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO channels (
                name,
                platform,
                api_key,
                is_active,
                valid_until,
                user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            insert.name,
            insert.platform as i32,
            insert.api_key,
            insert.is_active,
            insert.valid_until,
            insert.user_id.value(),
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(Key::new(id))
    }
}
