use std::sync::Arc;

use adapter_proc_macros::Repo;
use chrono::Utc;
use kernel_entities::{entities::auth::*, traits::Key};
use kernel_repositories::{auth::*, error::*, traits::*};
use ormx::{Delete, Patch, Table};

use crate::util::error::map_sqlx_error;
use crate::{database::SqlxDatabaseConnection, sqlx_ok};

#[derive(Repo)]
#[repo(
    table = "users",
    read(entity = "User", model = "models::UserModel"),
    insert(entity = "InsertUser", model = "models::InsertUserModel")
)]
    #[shaku(inject)]
    db: Arc<dyn SqlxDatabaseConnection>,
}

#[async_trait::async_trait]
impl UsersRepo for SqlxUsersRepo {
    async fn get_by_username(&self, username: &str) -> RepoResult<User> {
        sqlx_ok!(models::UserModel::by_username(self.db.get(), username).await)
    }

    async fn set_display_name(
        &self,
        id: &Key<User>,
        value: String,
    ) -> RepoResult<()> {
        sqlx_ok!(
            models::UpdateUserDisplayNameModel {
                display_name: value,
                updated_at: Utc::now()
            }
            .patch_row(self.db.get(), id.value())
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
        #[ormx(set)]
        pub display_name: String,
        #[ormx(get_one(&str))]
        pub username: String,
        pub is_active: bool,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    #[derive(ormx::Patch)]
    #[ormx(table_name = "users", table = UserModel, id = "id")]
    pub struct UpdateUserDisplayNameModel {
        pub display_name: String,
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
