use std::sync::Arc;

use adapter_proc_macros::Repo;
use chrono::{DateTime, Utc};
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{
    auth::{InsertUser, UsersRepo},
    error::RepoResult,
    traits::repo::*,
};
use ormx::{Delete, Table};
use shaku::Component;

use crate::{database::SqlxDatabaseConnection, sqlx_ok};
use crate::{sqlx_vec_ok, util::error::map_sqlx_error};

#[derive(Component, Repo)]
#[repo(
    table = "users",
    read(entity = "User", model = "models::UserModel"),
    insert(entity = "InsertUser", model = "models::InsertUserModel")
)]
#[shaku(interface = UsersRepo)]
pub struct SqlxUsersRepo {
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl UsersRepo for SqlxUsersRepo {
    async fn get_by_username(&self, username: &str) -> RepoResult<User> {
        sqlx_ok!(models::UserModel::by_username(self.db.get(), username).await)
    }

    async fn get_all(
        &self,
        pagination: (DateTime<Utc>, usize),
    ) -> RepoResult<Vec<User>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
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
        )
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities, traits::KeyType};
    use kernel_repositories::auth::InsertUser;

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

    impl Into<InsertUserModel> for InsertUser {
        fn into(self) -> InsertUserModel {
            InsertUserModel {
                id: uuid::Uuid::new_v4(),
                username: self.username,
                display_name: self.display_name,
                is_active: self.is_active,
            }
        }
    }

    generate_mapping!(entities::auth::User, UserModel, 6);
}
