use crate::{key_type, traits::*};
use kernel_proc_macros::*;

use super::{Account, Role};

#[entity(entity_type = "immutable")]
pub struct AccountRole {
    pub account_id: key_type!(Account),
    pub role_id: key_type!(Role),
    pub enabled: bool,
}
