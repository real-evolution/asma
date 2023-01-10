use kernel_proc_macros::entity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{entities::auth::User, traits::*};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum ChatState {
    Active,
    Archived,
    Closed,
}

#[entity]
#[derive(Clone, Debug, JsonSchema)]
pub struct Chat {
    pub label: Option<String>,
    pub state: ChatState,
    pub user_id: Key<User>,
}
