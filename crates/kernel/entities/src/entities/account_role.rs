use crate::traits::*;
use kernel_proc_macros::*;

use super::{Role, Account};

#[entity(entity_type = "immutable")]
pub struct AccountRole {
    pub account_id: <Account as Identifiable>::Key,
    pub role_id: <Role as Identifiable>::Key,
    pub enabled: bool,
}
