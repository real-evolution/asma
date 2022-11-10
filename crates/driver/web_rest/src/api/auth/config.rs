use common_macros::into_fn;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use validator::Validate;

pub(super) const API_TOKEN_CONFIG_SECTION: &str = "api.token";

into_fn!(default_issuer: String =>  "social.sgstel.com.ye".to_string());
into_fn!(default_audience: String => "social.sgstel.com.ye".to_string());
into_fn!(default_timeout_seconds: const i64 =>  5 * 60);

fn default_signing_key() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 128)
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ApiTokenConfig {
    #[serde(default = "default_issuer")]
    pub issuer: String,

    #[serde(default = "default_issuer")]
    pub audience: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_seconds")]
    pub timout_seconds: i64,

    #[serde(default = "default_signing_key")]
    pub signing_key: String,
}