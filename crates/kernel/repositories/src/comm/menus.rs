use derive_more::Constructor;
use kernel_entities::{
    entities::comm::{Bot, Menu, TriggerMatchingStrategy},
    traits::Key,
};

use crate::{error::RepoResult, traits::*};

#[async_trait::async_trait]
pub trait MenusRepo:
    Repo<Entity = Menu> + InsertRepo<InsertMenu> + ChildRepo<Bot> + Send + Sync
{
    async fn get_submenus(&self, id: &Key<Menu>) -> RepoResult<Vec<Menu>>;

    async fn get_with_submenus(
        &self,
        id: &Key<Menu>,
    ) -> RepoResult<(Menu, Vec<Menu>)>;

    async fn get_entry_menu_of(&self, bot_id: &Key<Bot>) -> RepoResult<Menu>;
}

#[derive(Constructor)]
pub struct InsertMenu {
    pub title: String,
    pub content: Option<String>,
    pub menu_trigger: String,
    pub matching_strategy: TriggerMatchingStrategy,
    pub is_active: bool,
    pub parent_menu_id: Option<Key<Menu>>,
    pub bot_id: Key<Bot>,
}
