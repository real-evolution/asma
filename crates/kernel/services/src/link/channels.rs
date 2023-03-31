use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};
use serde::{Deserialize, Serialize};

use super::message_passing::{ScopedTopicReader, ScopedTopicWriter};
use crate::error::AppResult;

#[async_trait::async_trait]
pub trait ChannelsService: Send + Sync {
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

    async fn start_channel(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> AppResult<()>;

    async fn stop_channel(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> AppResult<()>;

    async fn get_pipe_of(
        &self,
        user_id: &Key<User>,
        channel_id: Option<&Key<Channel>>,
    ) -> AppResult<ChannelPipe>;

    async fn get_pipe_of_all(&self) -> AppResult<ChannelPipe>;
}

#[derive(Clone)]
pub struct ChannelPipe {
    pub tx: Arc<dyn ScopedTopicWriter<OutgoingChannelUpdate>>,
    pub rx: Arc<dyn ScopedTopicReader<IncomingChannelUpdate>>,
}

#[derive(Clone)]
pub struct ReverseChannelPipe {
    pub tx: Arc<dyn ScopedTopicWriter<IncomingChannelUpdate>>,
    pub rx: Arc<dyn ScopedTopicReader<OutgoingChannelUpdate>>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelStatus {
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutgoingMessageUpdateKind {
    New { content: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IncomingMessageUpdateKind {
    New { content: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutgoingChannelUpdateKind {
    Message {
        platform_user_id: i64,
        kind: OutgoingMessageUpdateKind,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IncomingChannelUpdateKind {
    Message {
        platform_user_id: i64,
        kind: IncomingMessageUpdateKind,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelUpdate<Kind> {
    pub user_id: Key<User>,
    pub channel_id: Key<Channel>,
    pub kind: Kind,
}

pub type OutgoingChannelUpdate = ChannelUpdate<OutgoingChannelUpdateKind>;
pub type IncomingChannelUpdate = ChannelUpdate<IncomingChannelUpdateKind>;
