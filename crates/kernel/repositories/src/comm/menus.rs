use derive_more::Constructor;
use kernel_entities::{
    entities::comm::{Bot, Menu, TriggerMatchingStrategy},
    traits::Key,
};

use crate::traits::*;

#[async_trait::async_trait]
pub trait MenusRepo:
    Repo<Entity = Menu> + InsertRepo<InsertMenu> + ChildRepo<Bot> + Send + Sync
{
}

#[derive(Constructor)]
pub struct InsertMenu {
    pub title: String,
    pub content: Option<String>,
    pub menu_trigger: String,
    pub matching_strategy: TriggerMatchingStrategy,
    pub is_active: bool,
    pub parent_menu_id: Key<Menu>,
    pub bot_id: Key<Bot>,
}
