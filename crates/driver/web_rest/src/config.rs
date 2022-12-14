use aide::OperationIo;
use common_macros::into_fn;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use validator::Validate;

pub const API_CONFIG_SECTION: &str = "api";

into_fn!(default_issuer: String =>  "social.sgstel.com.ye".to_string());
into_fn!(default_audience: String => "social.sgstel.com.ye".to_string());
into_fn!(default_timeout_seconds: const i64 =>  5 * 60);

fn default_signing_key() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 128)
}

#[derive(Clone, Debug, Default, Deserialize, Validate, OperationIo)]
#[aide(input)]
pub struct ApiConfig {
    #[serde(default)]
    pub token: ApiTokenConfig,
    pub disable_root: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Validate)]
pub struct ApiTokenConfig {
    #[serde(default = "default_issuer")]
    pub issuer: String,

    #[serde(default = "default_issuer")]
    pub audience: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: i64,

    #[serde(default = "default_signing_key")]
    pub signing_key: String,
}
