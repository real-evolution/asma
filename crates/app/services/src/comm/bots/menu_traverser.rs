use std::sync::Arc;

use kernel_entities::{
    entities::comm::{Menu, TriggerMatchingStrategy},
    traits::Key,
};
use kernel_repositories::DataStore;
use kernel_services::error::AppResult;
use tokio::sync::RwLock;

struct MenuHierarchy {
    menu: Menu,
    sub: Vec<Menu>,
}

pub(super) struct MenuTraverser {
    data: Arc<dyn DataStore>,
    current: RwLock<MenuHierarchy>,
}

impl MenuTraverser {
    pub(super) async fn new(
        menu_id: &Key<Menu>,
        data: Arc<dyn DataStore>,
    ) -> AppResult<Self> {
        let hierarchy = Self::get_hierarchy(menu_id, &data).await?;

        Ok(Self {
            data,
            current: RwLock::new(hierarchy),
        })
    }

    pub(super) async fn process(&self, msg: &str) -> AppResult<bool> {
        if let Some(next_id) = self.get_next_menu(msg).await {
            *self.current.write().await =
                Self::get_hierarchy(&next_id, &self.data).await?;

            return Ok(true);
        }

        Ok(false)
    }

    pub(super) async fn to_formatted_string(&self) -> String {
        let current = self.current.read().await;

        let mut message = format!("- {}\n", current.menu.title);

        if let Some(ref content) = current.menu.content {
            message += &format!("  {content}\n");
        }

        if !current.sub.is_empty() {
            message += "\nSub-Menus:\n";

            for m in current.sub.iter() {
                message +=
                    &format!("- {} (with: {})\n", m.title, m.menu_trigger);
            }
        }

        message
    }

    async fn get_next_menu(&self, msg: &str) -> Option<Key<Menu>> {
        let msg = msg.to_lowercase();
        let cur = self.current.read().await;

        for m in cur.sub.iter() {
            let trigger = m.menu_trigger.to_lowercase();
            let matches = match m.matching_strategy {
                | TriggerMatchingStrategy::Full => msg == trigger,
                | TriggerMatchingStrategy::SubString => msg.contains(&trigger),
            };

            if matches {
                return Some(m.id.clone());
            }
        }

        None
    }

    async fn get_hierarchy(
        menu_id: &Key<Menu>,
        data: &Arc<dyn DataStore>,
    ) -> AppResult<MenuHierarchy> {
        let (menu, mut sub) =
            data.comm().menus().get_with_submenus(menu_id).await?;

        sub.sort_unstable_by_key(|m| {
            (m.matching_strategy.repr(), m.menu_trigger.len())
        });

        Ok(MenuHierarchy { menu, sub })
    }
}
