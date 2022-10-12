use crate::traits::*;
use domain_macros::*;

use super::{Role, Account};

#[derive(Entity)]
#[entity]
pub struct AccountRole {
    pub account_id: <Account as Identifiable>::Key,
    pub role_id: <Role as Identifiable>::Key,
    pub enabled: bool,
}
