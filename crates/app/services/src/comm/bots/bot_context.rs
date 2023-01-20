use std::{collections::HashMap, sync::Arc};

use kernel_entities::{
    entities::{comm::Menu, link::Instance},
    traits::Key,
};
use kernel_repositories::DataStore;
use kernel_services::error::AppResult;
use tokio::sync::RwLock;

use super::menu_traverser::MenuTraverser;

pub(super) struct BotContext {
    data: Arc<dyn DataStore>,
    entry: Menu,
    active: RwLock<HashMap<Key<Instance>, MenuTraverser>>,
}

impl BotContext {
    pub(super) fn new(entry: Menu, data: Arc<dyn DataStore>) -> Self {
        Self {
            data,
            entry,
            active: Default::default(),
        }
    }

    pub(super) async fn handle_message(
        &self,
        instance_id: &Key<Instance>,
        text: &str,
    ) -> AppResult<Option<String>> {
        let active = self.active.read().await;

        if !active.contains_key(instance_id) {
            let traverser =
                MenuTraverser::new(&self.entry.id, self.data.clone()).await?;

            self.active
                .write()
                .await
                .insert(instance_id.clone(), traverser);
        }

        if let Some(traverser) = active.get(instance_id) {
            if traverser.process(text).await? {
                return Ok(Some(traverser.to_formatted_string().await));
            }
        }

        Ok(None)
    }
}
