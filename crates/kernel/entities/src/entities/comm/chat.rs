use chrono::{DateTime, Utc};

use crate::entities::link::Instance;
use crate::{entities::auth::User, traits::Key};

#[derive(Clone, Debug)]
pub enum ChatState {
    Active,
    Archived,
    Closed,
}

#[derive(Clone, Debug)]
pub struct Chat {
    pub id: Key<Chat>,
    pub label: Option<String>,
    pub state: ChatState,
    pub user_id: Key<User>,
    pub instance_id: Key<Instance>,
    pub created_at: Option<DateTime<Utc>>,
}
