use derive_more::{From, Into};
use kernel_proc_macros::*;

use crate::{entities::auth::User, traits::*};

#[entity]
#[derive(Clone, Debug, From, Into, sqlx::FromRow)]
pub struct Peer {
    pub display_name: Option<String>,
    pub comment: Option<String>,
    pub is_active: bool,
    pub user_id: Key<User>,
}
