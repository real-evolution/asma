use chrono::{DateTime, Utc};
use kernel_proc_macros::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Account;
use crate::traits::*;

#[derive(Clone, Debug, JsonSchema, Deserialize, Serialize)]
pub enum OAuth2Provider {
    Google,
    Facebook,
    GitHub,
}

#[entity]
#[derive(Clone, Debug, JsonSchema)]
pub struct OAuth2Login {
    pub provider: OAuth2Provider,
    pub provided_name: Option<String>,
    pub provided_identifier: String,
    pub access_token: String,
    pub refresh_token: String,
    pub valid_until: DateTime<Utc>,
    pub account_id: Key<Account>,
}
