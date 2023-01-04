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

    fn start_channels(&self) -> BoxStream<'_, AppResult<()>>;
    fn stop_channels(&self) -> BoxStream<'_, AppResult<()>>;

    fn start_channels_of(
        &self,
        user_id: Key<User>,
    ) -> BoxStream<'_, AppResult<()>>;

    fn stop_channels_of(
        &self,
        user_id: Key<User>,
    ) -> BoxStream<'_, AppResult<()>>;

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
pub enum IncomingMessageUpdateKind {
    New {
        platform_message_id: String,
        content: Option<String>,
    },

    Edit {
        platform_message_id: String,
        content: Option<String>,
    },
}

#[derive(Debug)]
pub enum OutgoingMessageUpdateKind {
    New {
        content: String,
    },

    Edit {
        platform_message_id: String,
        content: Option<String>,
    },
}

#[derive(Debug)]
pub enum IncomingChannelUpdate {
    Message {
        platform_chat_id: String,
        platform_user_id: String,
        kind: IncomingMessageUpdateKind,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug)]
pub enum OutgoingChannelUpdate {
    Message {
        platform_chat_id: String,
        platform_user_id: String,
        kind: OutgoingMessageUpdateKind,
        timestamp: DateTime<Utc>,
    },
}
