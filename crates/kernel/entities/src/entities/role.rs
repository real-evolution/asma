use crate::traits::*;
use domain_proc_macros::*;

#[entity]
pub struct Role {
    pub friendly_name: Option<String>,
    pub code: String,
    pub enabled: bool,
}
