use std::sync::Arc;

use chrono::{DateTime, Utc};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{
    auth::{InsertUser, UsersRepo},
    error::RepoResult,
    traits::repo::Repo,
};
use ormx::Table;
use shaku::Component;

use crate::database::SqlxDatabaseConnection;
use crate::util::error::map_sqlx_error;

#[derive(Component)]
#[shaku(interface = UsersRepo)]
pub struct SqlxUsersRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl UsersRepo for SqlxUsersRepo {
    async fn get_by_username(&self, username: &str) -> RepoResult<User> {
        Ok(models::UserModel::by_username(self.db.get(), username)
            .await
            .map_err(map_sqlx_error)?
            .into())
    }

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<User>> {
        Ok(sqlx::query_as!(
            models::UserModel,
            r#"
            SELECT * FROM users
            WHERE created_at < $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            pagination.0,
            pagination.1 as i64
        )
        .fetch_all(self.db.get())
        .await
        .map_err(map_sqlx_error)?
        .into_iter()
        .map(|u| u.into())
        .collect())
    }

    async fn create(&self, insert: InsertUser) -> RepoResult<Key<User>> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (
                username,
                display_name,
                is_active)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            insert.username,
            insert.display_name,
            insert.is_active,
        )
        .fetch_one(self.db.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(Key::new(id))
    }

    async fn remove(&self, user_id: &Key<User>) -> RepoResult<()> {
        sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, user_id.value_ref())
            .execute(self.db.get())
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Repo<User> for SqlxUsersRepo {
    async fn get(&self, id: &Key<User>) -> RepoResult<User> {
        Ok(models::UserModel::get(self.db.get(), id.value())
            .await
            .map_err(map_sqlx_error)?
            .into())
    }

    async fn get_paginated(&self, params: (i64, i64)) -> RepoResult<Vec<User>> {
        Ok(
            models::UserModel::all_paginated(self.db.get(), params.0, params.1)
                .await
                .map_err(map_sqlx_error)?
                .into_iter()
                .map(|u| u.into())
                .collect(),
        )
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities, traits::KeyType};

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table, sqlx::FromRow)]
    #[ormx(table = "users", id = id, insertable, deletable)]
    pub struct UserModel {
        pub id: KeyType,
        pub display_name: String,
        #[ormx(get_one(&str))]
        pub username: String,
        pub is_active: bool,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    generate_mapping!(entities::auth::User, UserModel, 6);
}
