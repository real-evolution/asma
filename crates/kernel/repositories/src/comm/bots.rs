use derive_more::Constructor;
use kernel_entities::{
    entities::{auth::User, comm::Bot},
    traits::Key,
};

use crate::traits::*;

#[async_trait::async_trait]
pub trait BotsRepo:
    Repo<Entity = Bot> + InsertRepo<InsertBot> + Send + Sync
{
}

#[derive(Constructor)]
pub struct InsertBot {
    pub name: String,
    pub is_active: bool,
    pub user_id: Key<User>,
}
