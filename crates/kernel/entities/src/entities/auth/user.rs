use derive_more::{From, Into};
use kernel_proc_macros::*;

use crate::traits::*;

#[entity]
#[derive(Clone, Debug, From, Into)]
pub struct User {
    pub display_name: String,
    pub username: String,
    pub is_active: bool,
}
