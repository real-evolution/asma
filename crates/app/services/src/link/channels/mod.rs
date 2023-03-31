mod channel_state;
mod channel_stream;
mod telegram;

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use futures::{
    stream::{self, BoxStream},
    Stream, StreamExt,
};
use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};
use kernel_repositories::{error::RepoResult, DataStore};
use kernel_services::{
    error::AppResult,
    link::{
        channels::{
            ChannelPipe, ChannelStatus, ChannelsService, ReverseChannelPipe,
        },
        message_passing::{
            MessagePassingService, ScopedTopicReader, ScopedTopicWriter,
        },
    },
    Service,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;

use self::channel_state::ChannelState;

type ChannelStatesMap = HashMap<Key<Channel>, ChannelState>;
type UserChannelsMap = HashMap<Key<User>, ChannelStatesMap>;

const CHANNELS_TOPIC_NAME: &str = "channels";

pub struct AppChannelsService<IPC> {
    data: Arc<dyn DataStore>,
    ipc: Arc<IPC>,
    states: RwLock<UserChannelsMap>,
}

#[async_trait]
impl<Ipc: MessagePassingService> ChannelsService for AppChannelsService<Ipc> {
    async fn status(
        &self,
        id: &Key<Channel>,
    ) -> AppResult<Option<ChannelStatus>> {
        Ok(self.states.read().await.values().find_map(|chs| {
            chs.get(id).map(|s| ChannelStatus {
                started_at: s.started_at(),
            })
        }))
    }

    fn status_of<'a>(
        &'a self,
        user_id: &'a Key<User>,
    ) -> BoxStream<'a, (Key<Channel>, ChannelStatus)> {
        async_stream::stream! {
            let locked_states = self.states.read().await;
            let states = locked_states.get(user_id);

            match states {
                | Some(states) => {
                    for (channel_id, state) in states {
                        yield (channel_id.clone(), state.into());
                    }
                }
                | None => ()
            }
        }
        .boxed()
    }

    fn start_channels(&self) -> BoxStream<'_, AppResult<()>> {
        self.start_channels_stream(self.data.link().channels().stream_active())
    }

    fn stop_channels(&self) -> BoxStream<'_, AppResult<()>> {
        let states = self.states.blocking_read();

        stream::iter(states.keys().cloned().collect::<Vec<_>>())
            .map(|user_id| self.stop_channels_of(user_id))
            .flatten()
            .boxed()
    }

    fn start_channels_of(
        &self,
        user_id: Key<User>,
    ) -> BoxStream<'_, AppResult<()>> {
        self.start_channels_stream(
            self.data.link().channels().stream_active_of(user_id),
        )
    }

    fn stop_channels_of(
        &self,
        user_id: Key<User>,
    ) -> BoxStream<'_, AppResult<()>> {
        async_stream::stream! {
            let mut states = self.states.write().await;

            let Some(user_states) = states.get_mut(&user_id) else {
                return ;
            };

            for (channel_id, state) in user_states.drain() {
                let channel = state.channel();

                debug!(
                    "stopping {:?} channel #{} of #{} ({})",
                    channel.platform, channel.id, channel.user_id, channel.name,
                );

                match state.stop().await {
                    Ok(()) => yield Ok(()),
                    Err(err) => {
                        warn!("could not stop channel #{channel_id}: {err}");
                        yield Err(err)
                    }
                };

                yield Ok(());
            }

            states.remove(&user_id);
        }
        .boxed()
    }

    async fn start_channel(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> AppResult<()> {
        let channel = self
            .data
            .link()
            .channels()
            .get_of(user_id, channel_id)
            .await?;

        self.start_channel(channel).await
    }

    async fn stop_channel(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> AppResult<()> {
        let Some(state) = self.remove_state(user_id, channel_id).await else {
            debug!(
                "channel #{} of #{} is not running, skipping",
                channel_id, user_id
            );

            return Ok(());
        };

        debug!("stopping channel #{} of #{}", channel_id, user_id,);

        match state.stop().await {
            | Ok(()) => Ok(()),
            | Err(err) => {
                warn!("could not stop channel #{}: {err}", channel_id);
                Err(err)
            }
        }
    }

    async fn get_pipe_of(
        &self,
        user_id: &Key<User>,
        channel_id: Option<&Key<Channel>>,
    ) -> AppResult<ChannelPipe> {
        self.create_pipe(Some(user_id), channel_id).await
    }

    async fn get_pipe_of_all(&self) -> AppResult<ChannelPipe> {
        self.create_pipe(None, None).await
    }
}

#[async_trait]
impl<Ipc: MessagePassingService> Service for AppChannelsService<Ipc> {
    async fn initialize(self: Arc<Self>) -> AppResult<()> {
        self.get_pipe_of_all().await?;

        debug!("starting channels");

        let mut channels = self.start_channels();

        while let Some(res) = channels.next().await {
            if let Err(err) = res {
                warn!("error starting channel: {err}")
            }
        }

        Ok(())
    }
}

impl<IPC: MessagePassingService> AppChannelsService<IPC> {
    pub fn new(data: Arc<dyn DataStore>, ipc: Arc<IPC>) -> Self {
        Self {
            data,
            ipc,
            states: Default::default(),
        }
    }

    async fn start_channel(&self, channel: Channel) -> AppResult<()> {
        if self.has_state(&channel.user_id, &channel.id).await {
            debug!(
                "channel #{} of #{} ({}) is already running, skipping",
                channel.id, channel.user_id, channel.name,
            );

            return Ok(());
        }
        debug!(
            "starting {:?} channel #{} of#{} ({})",
            channel.platform, channel.id, channel.user_id, channel.name,
        );

        let pipe = self
            .create_reverse_pipe(&channel.user_id, &channel.id)
            .await?;
        let state = ChannelState::new(channel, pipe)?;

        match state.run().await {
            | Ok(_) => {
                self.append_state(&state.channel().user_id.clone(), state)
                    .await;

                Ok(())
            }

            | Err(err) => {
                warn!("could not start channel: {err}");
                Err(err)
            }
        }
    }

    fn start_channels_stream<'a, S>(
        &'a self,
        channels: S,
    ) -> BoxStream<'a, AppResult<()>>
    where
        S: Stream<Item = RepoResult<Channel>> + Send + 'a,
    {
        channels
            .then(|c| async {
                let channel = match c {
                    | Ok(channel) => channel,
                    | Err(err) => {
                        warn!("could not load channel: {err}");
                        return Err(err.into());
                    }
                };

                self.start_channel(channel).await
            })
            .boxed()
    }

    async fn append_state(&self, user_id: &Key<User>, state: ChannelState) {
        let mut states = self.states.write().await;

        match states.get_mut(user_id) {
            | Some(channels) => {
                channels.insert(state.channel().id.clone(), state);
            }

            | None => {
                let mut user_states = HashMap::default();
                user_states.insert(state.channel().id.clone(), state);

                states.insert(user_id.clone(), user_states);
            }
        }
    }

    async fn remove_state(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> Option<ChannelState> {
        let mut states = self.states.write().await;

        if let Some(user_states) = states.get_mut(user_id) {
            return user_states.remove(channel_id);
        }

        None
    }

    async fn has_state(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> bool {
        let states = self.states.read().await;

        match states.get(user_id) {
            | Some(channels) => channels.contains_key(channel_id),
            | None => false,
        }
    }

    async fn create_pipe(
        &self,
        user_id: Option<&Key<User>>,
        channel_id: Option<&Key<Channel>>,
    ) -> AppResult<ChannelPipe> {
        let user_id = user_id.map_or("*".to_owned(), ToString::to_string);
        let channel_id = channel_id.map_or("*".to_owned(), ToString::to_string);

        let (tx, rx) = self
            .create_pipe_parts(
                &format!("{user_id}.{channel_id}.out"),
                &format!("{user_id}.{channel_id}.in"),
            )
            .await?;

        Ok(ChannelPipe { tx, rx })
    }

    async fn create_reverse_pipe(
        &self,
        user_id: &Key<User>,
        channel_id: &Key<Channel>,
    ) -> AppResult<ReverseChannelPipe> {
        let (tx, rx) = self
            .create_pipe_parts(
                &format!("{user_id}.{channel_id}.in"),
                &format!("{user_id}.{channel_id}.out"),
            )
            .await?;

        Ok(ReverseChannelPipe { tx, rx })
    }

    async fn create_pipe_parts<
        T: Serialize + Send + Sync + 'static,
        R: DeserializeOwned + Send + Sync + 'static,
    >(
        &self,
        tx_key: &str,
        rx_key: &str,
    ) -> AppResult<(Arc<dyn ScopedTopicWriter<T>>, Arc<dyn ScopedTopicReader<R>>)>
    {
        let tx = self
            .ipc
            .get_topic_writer(CHANNELS_TOPIC_NAME)
            .await?
            .scoped(tx_key);

        let rx = self
            .ipc
            .get_topic_reader(CHANNELS_TOPIC_NAME)
            .await?
            .scoped(rx_key);

        Ok((tx, rx))
    }
}

impl From<&ChannelState> for ChannelStatus {
    fn from(val: &ChannelState) -> Self {
        ChannelStatus {
            started_at: val.started_at(),
        }
    }
}
