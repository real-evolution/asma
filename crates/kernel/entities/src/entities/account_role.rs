use crate::traits::*;
use domain_macros::*;

#[derive(Entity)]
#[entity]
pub struct AccountRole {
    pub friendly_name: Option<String>,
    pub code: String,
    pub enabled: bool,
}
