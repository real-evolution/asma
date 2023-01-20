mod bot_cluster;
mod bot_context;
mod menu_traverser;

use std::{collections::HashMap, sync::Arc};

use futures::TryStreamExt;
use kernel_entities::{entities::auth::User, traits::Key};
use kernel_repositories::DataStore;
use kernel_services::{
    comm::{bots::BotsService, chats::ChatsService},
    error::AppResult,
    Service,
};
use tokio::sync::RwLock;

use self::bot_cluster::BotCluster;

pub struct AppBotsService {
    data: Arc<dyn DataStore>,
    chats_svc: Arc<dyn ChatsService>,
    clusters: RwLock<HashMap<Key<User>, Arc<BotCluster>>>,
}

#[async_trait::async_trait]
impl BotsService for AppBotsService {}

impl AppBotsService {
    pub fn new(
        data: Arc<dyn DataStore>,
        chats_svc: Arc<dyn ChatsService>,
    ) -> Self {
        Self {
            data,
            chats_svc,
            clusters: Default::default(),
        }
    }

    async fn ensure_clustre_created(
        &self,
        user_id: &Key<User>,
    ) -> AppResult<Arc<BotCluster>> {
        if let Some(cluster) = self.clusters.read().await.get(user_id) {
            return Ok(cluster.clone());
        }

        debug!("creating bot cluster for user #{user_id}");

        let cluster = Arc::new(BotCluster::new(
            self.data.clone(),
            user_id,
            self.chats_svc.clone(),
        ));

        self.clusters
            .write()
            .await
            .insert(user_id.clone(), cluster.clone());

        Ok(cluster)
    }
}

#[async_trait::async_trait]
impl Service for AppBotsService {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        debug!("loading bots");

        let mut bots = self.data.comm().bots().stream_active();

        while let Some(bot) = bots.try_next().await? {
            let cluster = self.ensure_clustre_created(&bot.user_id).await?;

            cluster.append_bot(bot).await?;
        }

        debug!("starting bot clusters");

        let clusters = self.clusters.read().await;

        for (user_id, cluster) in clusters.iter() {
            if !cluster.is_running().await {
                continue;
            }

            debug!("starting bot cluster for user #{user_id}");

            if let Err(err) = cluster.clone().start().await {
                warn!("error starting bot cluster: {err}")
            }
        }

        Ok(())
    }
}
