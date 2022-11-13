use std::sync::Arc;

use kernel_entities::entities::link::*;
use kernel_repositories::{
    error::RepoResult,
    link::{ChannelsRepo, InsertChannel},
};
use shaku::Component;

use crate::{database::SqlxDatabaseConnection, util::map_sqlx_error};

#[derive(Component)]
#[shaku(interface = ChannelsRepo)]
pub struct SqlxChannelsRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl ChannelsRepo for SqlxChannelsRepo {
    async fn get_by_id(&self, id: &ChannelKey) -> RepoResult<Channel> {
        Ok(
            sqlx::query_as::<_, Channel>(
                "SELECT * FROM channels WHERE id = $1",
            )
            .bind(id)
            .fetch_one(self.db.get())
            .await
            .map_err(map_sqlx_error)?,
        )
    }

    async fn create(&self, insert: InsertChannel) -> RepoResult<ChannelKey> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO channels (
                name,
                api_key,
                is_active,
                valid_until,
                user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            insert.name,
            insert.api_key,
            insert.is_active,
            insert.valid_until,
            insert.user_id.0,
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(ChannelKey(id))
    }
}
