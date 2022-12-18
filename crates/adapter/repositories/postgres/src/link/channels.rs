use async_stream::stream;
use chrono::{DateTime, Utc};
use futures::{stream::BoxStream, TryStreamExt};
use kernel_entities::{
    entities::{auth::User, link::*},
    traits::Key,
};
use kernel_repositories::{error::RepoResult, link::*, traits::*};
use ormx::{Delete, Table};
use proc_macros::Repo;

use crate::{
    database::SqlxPool,
    sqlx_ok,
    sqlx_stream_ok,
    sqlx_vec_ok,
    util::error::map_sqlx_error,
};

#[derive(Repo)]
#[repo(
    table = "channels",
    read(entity = "Channel", model = "models::ChannelModel"),
    insert(entity = "InsertChannel", model = "models::InsertChannelModel")
)]
pub(crate) struct SqlxChannelsRepo(pub SqlxPool);

#[async_trait::async_trait]
impl ChannelsRepo for SqlxChannelsRepo {
    async fn stream_active<'a>(&'a self) -> BoxStream<'a, RepoResult<Channel>> {
        
        sqlx_stream_ok!(sqlx::query_as!(
            models::ChannelModel,
            r#"
                SELECT * FROM channels
                WHERE is_active = TRUE AND
                      valid_until != NULL AND
                      valid_until > now()
                ORDER BY created_at
                "#
        )
        .fetch(self.0.get()))
    }

    async fn stream_active_of<'a>(
        &'a self,
        user_id: Key<User>,
    ) -> BoxStream<'a, RepoResult<Channel>> {
        sqlx_stream_ok!(sqlx::query_as!(
            models::ChannelModel,
            r#"
                SELECT * FROM channels
                WHERE user_id = $1 AND
                      is_active = TRUE AND
                      valid_until != NULL AND
                      valid_until > now()
                ORDER BY created_at
                "#,
            user_id.value(),
        )
        .fetch(self.0.get()))
    }
}

#[async_trait::async_trait]
impl ChildRepo<User> for SqlxChannelsRepo {
    async fn get_paginated_of(
        &self,
        user_id: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::ChannelModel,
                r#"
                SELECT * FROM channels
                WHERE user_id = $1 AND created_at <= $2
                ORDER BY created_at
                LIMIT $3
                "#,
                user_id.value_ref(),
                before,
                limit as i64
            )
            .fetch_all(self.0.get())
            .await
        )
    }

    async fn get_of(
        &self,
        user_id: &Key<User>,
        id: &Key<Self::Entity>,
    ) -> RepoResult<Self::Entity> {
        sqlx_ok!(
            sqlx::query_as!(
                models::ChannelModel,
                r#"SELECT * FROM channels WHERE id = $1 AND user_id = $2"#,
                id.value_ref(),
                user_id.value_ref()
            )
            .fetch_one(self.0.get())
            .await
        )
    }

    async fn remove_of(
        &self,
        user_id: &Key<User>,
        id: &Key<Self::Entity>,
    ) -> RepoResult<()> {
        sqlx::query_as!(
            models::ChannelModel,
            r#"DELETE FROM channels WHERE id = $1 AND user_id = $2"#,
            id.value_ref(),
            user_id.value_ref()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

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
