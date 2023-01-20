use std::{collections::HashMap, sync::Arc};

use kernel_entities::{
    entities::{
        comm::{Menu, Message},
        link::Instance,
    },
    traits::Key,
};
use kernel_repositories::DataStore;
use kernel_services::error::AppResult;
use tokio::sync::RwLock;

use super::menu_traverser::MenuTraverser;

pub(super) struct BotContext {
    data: Arc<dyn DataStore>,
    root: Menu,
    active: RwLock<HashMap<Key<Instance>, MenuTraverser>>,
}

impl BotContext {
    pub(super) async fn handle_message(
        &self,
        msg: &Message,
    ) -> AppResult<Option<String>> {
        let Some(ref text) = msg.text else {
            return Ok(None);
        };

        let active = self.active.read().await;

        if !active.contains_key(&msg.instance_id) {
            let traverser =
                MenuTraverser::new(&self.root.id, self.data.clone()).await?;

            self.active
                .write()
                .await
                .insert(msg.instance_id.clone(), traverser);
        }

        if let Some(traverser) = active.get(&msg.instance_id) {
            if traverser.process(text).await? {
                return Ok(Some(traverser.to_formatted_string().await));
            }
        }

        Ok(None)
    }
}
