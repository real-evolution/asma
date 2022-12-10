use proc_macros::Repo;
use kernel_entities::{entities::link::*, traits::Key};
use kernel_repositories::{error::RepoResult, link::*, traits::*};
use ormx::{Delete, Table};

use crate::{database::SqlxPool, util::error::map_sqlx_error};

#[derive(Repo)]
#[repo(
    table = "instances",
    read(entity = "Instance", model = "models::InstanceModel")
)]
pub(crate) struct SqlxInstancesRepo(pub SqlxPool);

#[async_trait::async_trait]
impl InstancesRepo for SqlxInstancesRepo {}

mod models {
    use super::*;

    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::traits::KeyType;
    use ormx::Table;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, Table)]
    #[ormx(table = "instances", id = id, insertable, deletable)]
    pub struct InstanceModel {
        pub id: KeyType,
        pub platform_identifier: String,
        #[ormx(default, set)]
        pub display_name: Option<String>,
        #[ormx(default, set)]
        pub phone_number: Option<String>,
        #[ormx(default, set)]
        pub last_active: Option<DateTime<Utc>>,
        pub channel_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
    }

    generate_mapping!(Instance, InstanceModel, 7);
}
