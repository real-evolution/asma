use derive_more::{From, Into};
use kernel_proc_macros::*;
use schemars::JsonSchema;

use super::Instance;
use crate::{entities::auth::User, traits::*};

#[entity]
#[derive(Clone, Debug, From, Into, JsonSchema)]
pub struct InstanceGroup {
    pub display_name: Option<String>,
    pub comment: Option<String>,
    pub user_id: Key<User>,
}

#[entity(entity_type = "immutable")]
pub struct InstanceGroupMembership {
    pub group_id: Key<InstanceGroup>,
    pub instance_id: Key<Instance>,
}
