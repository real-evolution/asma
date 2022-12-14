mod handlers;

use std::{collections::HashMap, sync::Arc};

use async_stream::stream;
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};
use kernel_repositories::DataStore;
use kernel_services::{
    error::AppResult,
    link::{channels::ChannelsService, models::ChannelStatus},
};

use self::handlers::ChannelHandler;

pub struct AppChannelsService {
    _data: Arc<dyn DataStore>,
    handlers:
        HashMap<Key<User>, HashMap<Key<Channel>, Arc<dyn ChannelHandler>>>,
}

#[async_trait::async_trait]
impl ChannelsService for AppChannelsService {
    async fn start_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>> {
        let _ = Box::pin(stream! {});

        todo!()
    }

    async fn stop_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>> {
        todo!()
    }

    async fn status(
        &self,
        id: &Key<Channel>,
    ) -> AppResult<Option<ChannelStatus>> {
        for user_channels in self.handlers.values() {
            let Some(channel) = user_channels.get(id) else {
                continue;
            };

            return channel.status().await;
        }

        Ok(None)
    }

    async fn status_of<'a>(
        &'a self,
        user_id: &'a Key<User>,
    ) -> AppResult<BoxStream<'a, (Key<Channel>, ChannelStatus)>> {
        Ok(Box::pin(stream! {
        if let Some(channels) = self.handlers.get(user_id) {
            for (id, state) in channels {
                if let Ok(Some(status)) = state.status().await {
                    yield (id.clone(), status);
                }
            }
        }
        }))
    }
}
