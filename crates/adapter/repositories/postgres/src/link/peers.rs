use proc_macros::Repo;
use kernel_entities::{entities::link::*, traits::Key};
use kernel_repositories::{error::RepoResult, link::*, traits::*};
use ormx::{Delete, Table};

use crate::{database::SqlxPool, util::error::map_sqlx_error};

#[derive(Repo)]
#[repo(table = "peers", read(entity = "Peer", model = "models::PeerModel"))]
pub(crate) struct SqlxPeersRepo(pub SqlxPool);

#[async_trait::async_trait]
impl PeersRepo for SqlxPeersRepo {}

mod models {
    use super::*;

    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::traits::KeyType;
    use ormx::Table;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, Table)]
    #[ormx(table = "peers", id = id, insertable, deletable)]
    pub struct PeerModel {
        pub id: KeyType,
        pub display_name: Option<String>,
        pub comment: Option<String>,
        #[ormx(set)]
        pub is_active: bool,
        pub user_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    generate_mapping!(Peer, PeerModel, 7);
}
