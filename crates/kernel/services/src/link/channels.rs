use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};
use serde::Serialize;

use super::message_passing::Topic;
use crate::error::AppResult;

#[async_trait::async_trait]
pub trait ChannelsService<IpcTopic: Topic>: Send + Sync {
    async fn status(
        &self,
        id: &Key<Channel>,
    ) -> AppResult<Option<ChannelStatus>>;

    fn status_of<'a>(
        &'a self,
        user_id: &'a Key<User>,
    ) -> BoxStream<'a, (Key<Channel>, ChannelStatus)>;

    fn start_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>>;
    fn stop_channels<'a>(&'a self) -> BoxStream<'a, AppResult<()>>;

    fn start_channels_of<'a>(
        &'a self,
        user_id: Key<User>,
    ) -> BoxStream<'a, AppResult<()>>;

    fn stop_channels_of<'a>(
        &'a self,
        user_id: Key<User>,
    ) -> BoxStream<'a, AppResult<()>>;

    async fn get_pipe_of(
        &self,
        user_id: &Key<User>,
        channel_id: Option<&Key<Channel>>,
    ) -> AppResult<ChannelPipe<IpcTopic>>;

    async fn get_pipe_of_all(&self) -> AppResult<ChannelPipe<IpcTopic>>;
}

pub struct ChannelPipe<IpcTopic: Topic> {
    pub tx: Arc<IpcTopic>,
    pub rx: Arc<IpcTopic>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelStatus {
    pub started_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum MessageUpdateKind {
    New {
        platform_message_id: Option<String>,
        content: Option<String>,
    },

    Edit {
        platform_message_id: String,
        content: Option<String>,
    },

    Delete {
        platform_message_id: String,
    },
}

#[derive(Debug)]
pub struct MessageUpdate {
    pub chat_id: String,
    pub by_id: String,
    pub kind: MessageUpdateKind,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum ChannelUpdateKind {
    Message(MessageUpdate),
}

pub struct ChannelUpdate {
    pub user_id: Key<User>,
    pub channel_id: Key<Channel>,
    pub kind: ChannelUpdateKind,
}
