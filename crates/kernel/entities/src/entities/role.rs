use crate::traits::*;
use kernel_proc_macros::*;

#[entity]
pub struct Role {
    pub friendly_name: Option<String>,
    pub code: String,
    pub enabled: bool,
}
