use std::{collections::HashMap, sync::Arc};

use futures::TryStreamExt;
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Bot, MessageDirection},
    },
    traits::Key,
};
use kernel_repositories::{error::RepoError, DataStore};
use kernel_services::{
    comm::chats::{ChatEventKind, ChatsService},
    error::AppResult,
};
use tokio::{
    sync::{Mutex, RwLock},
    task::JoinHandle,
};

use super::bot_context::BotContext;

pub(super) struct BotCluster {
    data: Arc<dyn DataStore>,
    bots: RwLock<HashMap<Key<Bot>, BotContext>>,
    user_id: Key<User>,
    chat_svc: Arc<dyn ChatsService>,
    watch_task: Mutex<Option<JoinHandle<()>>>,
}

impl BotCluster {
    pub(super) fn new(
        data: Arc<dyn DataStore>,
        user_id: &Key<User>,
        chat_svc: Arc<dyn ChatsService>,
    ) -> Self {
        Self {
            data,
            bots: Default::default(),
            user_id: user_id.clone(),
            chat_svc,
            watch_task: Default::default(),
        }
    }

    pub(super) async fn append_bot(&self, bot: Bot) -> AppResult<()> {
        if self.bots.read().await.contains_key(&bot.id) {
            warn!("an attempt to add an existing bot cluster, ignoring");
            return Ok(());
        }

        let entry =
            match self.data.comm().menus().get_entry_menu_of(&bot.id).await {
                | Ok(entry) => entry,
                | Err(err) => {
                    return match err {
                        | RepoError::NotFound => {
                            warn!("bot #{} has no menu, skipping", bot.id);
                            Ok(())
                        }
                        | _ => Err(err.into()),
                    };
                }
            };

        let context = BotContext::new(entry, self.data.clone());

        info!(
            "adding bot #{} to bot cluster of user #{}",
            bot.id, bot.user_id
        );

        self.bots.write().await.insert(bot.id.clone(), context);

        Ok(())
    }

    pub(super) async fn start(self: Arc<Self>) -> AppResult<()> {
        if self.watch_task.lock().await.is_some() {
            return Ok(());
        }

        let this = self.clone();

        *self.watch_task.lock().await = Some(tokio::spawn(async move {
            if let Err(err) = this.clone().watch_user_messages().await {
                error!(
                    "an error occured while running bot cluster of user #{}: \
                     {err}",
                    this.user_id
                );
            }

            *this.watch_task.lock().await = None;
        }));

        Ok(())
    }

    pub(super) async fn is_running(&self) -> bool {
        self.watch_task.lock().await.is_some()
    }

    async fn watch_user_messages(self: Arc<Self>) -> AppResult<()> {
        let mut stream = self.chat_svc.watch_user_chats(&self.user_id).await?;

        while let Some(event) = stream.try_next().await? {
            match event.kind {
                | ChatEventKind::MessageAdded {
                    id,
                    text,
                    instance_id,
                    direction,
                    created_at,
                } => {
                    if let MessageDirection::Outgoing = direction {
                        continue;
                    }

                    let Some(text) = text else {
                        info!("ignoring empty message from instance #{} on bot cluster of user #{}", instance_id, self.user_id);
                        continue;
                    };

                    let bots = self.bots.read().await;

                    for (bot_id, ctx) in bots.iter() {
                        if let Some(resp) =
                            ctx.handle_message(&instance_id, &text).await?
                        {
                            info!(
                                "sending response from bot #{} to instance \
                                 #{} to message #{} sent at {}",
                                bot_id, instance_id, id, created_at
                            );

                            self.chat_svc
                                .send_message(&event.chat_id, resp)
                                .await?;

                            break;
                        }
                    }
                }
            };
        }

        Ok(())
    }
}
