use crate::traits::*;
use domain_macros::*;

#[derive(MutableEntity)]
#[mutable_entity]
pub struct Role {
    pub friendly_name: Option<String>,
    pub code: String,
    pub enabled: bool,
}
