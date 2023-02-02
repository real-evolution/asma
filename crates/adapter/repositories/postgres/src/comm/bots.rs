use chrono::{DateTime, Utc};
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use kernel_entities::{
    entities::{auth::User, comm::Bot},
    traits::Key,
};
use kernel_repositories::{
    comm::{BotsRepo, InsertBot},
    error::RepoResult,
    traits::*,
};
use ormx::{Delete, Table};
use proc_macros::Repo;

use crate::{
    database::SqlxPool,
    sqlx_ok,
    sqlx_vec_ok,
    util::error::map_sqlx_error,
};

#[derive(Repo)]
#[repo(
    table = "bots",
    read(entity = "Bot", model = "models::BotModel"),
    insert(entity = "InsertBot", model = "models::InsertBotModel")
)]
pub(crate) struct SqlxBotsRepo(pub SqlxPool);

#[async_trait::async_trait]
impl BotsRepo for SqlxBotsRepo {
    fn stream_active(&self) -> BoxStream<'_, RepoResult<Bot>> {
        sqlx::query_as!(
            models::BotModel,
            "SELECT * FROM bots WHERE is_active = TRUE"
        )
        .fetch(self.0.get())
        .map_ok(Into::into)
        .map_err(map_sqlx_error)
        .boxed()
    }
}

#[async_trait::async_trait]
impl ChildRepo<User> for SqlxBotsRepo {
    async fn get_paginated_of(
        &self,
        user_id: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::BotModel,
                r#"
                SELECT * FROM bots
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
        id: &Key<Bot>,
    ) -> RepoResult<Bot> {
        sqlx_ok!(
            sqlx::query_as!(
                models::BotModel,
                r#"SELECT * FROM bots WHERE id = $1 AND user_id = $2"#,
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
        id: &Key<Bot>,
    ) -> RepoResult<()> {
        sqlx::query_as!(
            models::BotModel,
            r#"DELETE FROM bots WHERE id = $1 AND user_id = $2"#,
            id.value_ref(),
            user_id.value_ref()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

#[async_trait::async_trait()]
impl StatsRepo<User> for SqlxBotsRepo {
    async fn get_stats_for(
        &self,
        parent_key: &Key<User>,
    ) -> RepoResult<StatsPair> {
        sqlx::query!(
            r#"
            SELECT
                COUNT(id) AS "total!",
                (
                    SELECT COUNT(id) FROM bots
                    WHERE user_id = $1 AND is_active = TRUE
                ) AS "active!"
            FROM bots
            WHERE user_id = $1
            "#,
            parent_key.value_ref(),
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)
        .map(|r| StatsPair::new(r.total as u64, r.active as u64))
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::comm::Bot, traits::KeyType};
    use kernel_repositories::comm::InsertBot;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "bots", id = id, insertable, deletable)]
    pub struct BotModel {
        #[ormx(default)]
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
                name: val.name,
                is_active: val.is_active,
                user_id: val.user_id.value(),
            }
        }
    }

    generate_mapping!(Bot, BotModel, 6);
}
