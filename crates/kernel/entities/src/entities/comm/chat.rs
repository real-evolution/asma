use chrono::{DateTime, Utc};
use schemars::JsonSchema;

use crate::entities::link::{Instance, Peer};
use crate::traits::Entity;
use crate::{entities::auth::User, traits::Key};

#[derive(Clone, Debug, JsonSchema)]
pub enum ChatState {
    Active,
    Archived,
    Closed,
}

#[derive(Clone, Debug, JsonSchema)]
pub struct Chat {
    pub id: Key<Chat>,
    pub label: Option<String>,
    pub state: ChatState,
    pub peer_id: Key<Peer>,
    pub instance_id: Key<Instance>,
    pub user_id: Key<User>,
    pub started_at: DateTime<Utc>,
}

impl Entity for Chat {
    fn id(&self) -> &Key<Self> {
        &self.id
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.started_at
    }
}
