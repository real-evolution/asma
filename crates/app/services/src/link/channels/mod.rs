mod channel_state;
mod channel_stream;
mod telegram;

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use futures::{
    stream::{self, BoxStream},
    Stream,
    StreamExt,
};
use kernel_entities::{
    entities::{auth::User, link::Channel},
    traits::Key,
};
use kernel_repositories::{error::RepoResult, DataStore};
use kernel_services::{
    error::AppResult,
    link::{
        channels::{ChannelPipe, ChannelStatus, ChannelsService},
        message_passing::MessagePassingService,
    },
    Service,
};
use tokio::sync::RwLock;

use self::channel_state::ChannelState;

type ChannelStatesMap = HashMap<Key<Channel>, ChannelState>;
type UserChannelsMap = HashMap<Key<User>, ChannelStatesMap>;

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
                | None => return ()
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
                return ();
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

    async fn get_pipe_of(
        &self,
        user_id: &Key<User>,
        channel_id: Option<&Key<Channel>>,
    ) -> AppResult<ChannelPipe> {
        let channel_segment = channel_id
            .map(|i| i.value().to_string())
            .unwrap_or_else(|| "*".to_owned());

        let key = format!("{}.{}", user_id.value_ref(), channel_segment);

        self.create_pipe(&key).await
    }

    async fn get_pipe_of_all(&self) -> AppResult<ChannelPipe> {
        self.create_pipe("#").await
    }
}

#[async_trait]
impl<Ipc: MessagePassingService> Service for AppChannelsService<Ipc> {
    async fn initialize(&self) -> AppResult<()> {
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

                debug!(
                    "starting {:?} channel #{} of#{} ({})",
                    channel.platform, channel.id, channel.user_id, channel.name,
                );

                let pipe = self
                    .get_pipe_of(&channel.user_id, Some(&channel.id))
                    .await?;
                let state = ChannelState::new(channel, pipe)?;

                match state.run().await {
                    | Ok(_) => {
                        self.append_state(
                            &state.channel().user_id.clone(),
                            state,
                        )
                        .await;

                        Ok(())
                    }

                    | Err(err) => {
                        warn!("could not start channel: {err}");
                        Err(err)
                    }
                }
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

    async fn create_pipe(&self, key: &str) -> AppResult<ChannelPipe> {
        let tx = self.ipc.get_topic(&format!("channels.{key}-out")).await?;
        let rx = self.ipc.get_topic(&format!("channels.{key}-in")).await?;

        Ok(ChannelPipe {
            incoming: tx,
            outgoing: rx,
        })
    }
}

impl From<&ChannelState> for ChannelStatus {
    fn from(val: &ChannelState) -> Self {
        ChannelStatus {
            started_at: val.started_at(),
        }
    }
}
