use enum_repr::EnumRepr;
use kernel_proc_macros::entity;
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Serialize};

use super::Bot;
use crate::traits::*;

#[EnumRepr(type = "i32")]
#[derive(Clone, Copy, Debug, JsonSchema_repr, Deserialize, Serialize)]
pub enum TriggerMatchingStrategy {
    Full = 0,
    SubString = 1,
    RegEx = 2,
}

#[entity]
#[derive(Clone, Debug, JsonSchema)]
pub struct Menu {
    title: String,
    content: Option<String>,
    trigger: String,
    matching_strategy: TriggerMatchingStrategy,
    parent_menu_id: Option<Key<Menu>>,
    bot_id: Key<Bot>,
}
