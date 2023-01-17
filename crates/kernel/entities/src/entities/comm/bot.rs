use derive_more::{From, Into};
use kernel_proc_macros::*;
use schemars::JsonSchema;

use crate::{entities::auth::User, traits::*};

#[entity]
#[derive(Clone, Debug, From, Into, JsonSchema)]
pub struct Bot {
    pub name: String,
    pub is_active: bool,
    pub user_id: Key<User>,
}
