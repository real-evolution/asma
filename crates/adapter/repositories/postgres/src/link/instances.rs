use kernel_entities::{entities::link::*, traits::Key};
use kernel_repositories::{error::RepoResult, link::*, traits::*};
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
    table = "instances",
    read(entity = "Instance", model = "models::InstanceModel"),
    insert(entity = "InsertInstance", model = "models::InsertInstanceModel")
)]
pub(crate) struct SqlxInstancesRepo(pub SqlxPool);

#[async_trait::async_trait]
impl InstancesRepo for SqlxInstancesRepo {
    async fn get_by_platform_identifier(
        &self,
        channel_id: &Key<Channel>,
        identifier: i64,
    ) -> RepoResult<Instance> {
        sqlx_ok!(
            sqlx::query_as!(
                models::InstanceModel,
                r#"SELECT * FROM instances
                   WHERE channel_id = $1 AND platform_identifier = $2"#,
                channel_id.value_ref(),
                identifier
            )
            .fetch_one(self.0.get())
            .await
        )
    }

    async fn get_by_platform_username(
        &self,
        channel_id: &Key<Channel>,
        username: &str,
    ) -> RepoResult<Instance> {
        sqlx_ok!(
            sqlx::query_as!(
                models::InstanceModel,
                r#"SELECT * FROM instances
                   WHERE channel_id = $1 AND platform_username = $2"#,
                channel_id.value_ref(),
                username
            )
            .fetch_one(self.0.get())
            .await
        )
    }

    async fn get_all(
        &self,
        channel_id: &Key<Channel>,
    ) -> RepoResult<Vec<Instance>> {
        sqlx_vec_ok!(
            models::InstanceModel::by_channel_id(
                self.0.get(),
                channel_id.value_ref(),
            )
            .await
        )
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::traits::KeyType;
    use ormx::Table;

    use super::*;
    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, Table)]
    #[ormx(table = "instances", id = id, insertable, deletable)]
    pub struct InstanceModel {
        pub id: KeyType,
        pub platform_identifier: i64,
        pub platform_username: String,
        #[ormx(default, set)]
        pub display_name: Option<String>,
        #[ormx(default, set)]
        pub phone_number: Option<String>,
        #[ormx(default, set)]
        pub last_active: Option<DateTime<Utc>>,
        #[ormx(get_many)]
        pub channel_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default)]
        pub updated_at: DateTime<Utc>,
    }

    impl From<InsertInstance> for InsertInstanceModel {
        fn from(val: InsertInstance) -> Self {
            InsertInstanceModel {
                id: uuid::Uuid::new_v4(),
                platform_identifier: val.platform_identifier,
                platform_username: val.platform_username,
                channel_id: val.channel_id.value(),
            }
        }
    }

    generate_mapping!(Instance, InstanceModel, 9);
}
