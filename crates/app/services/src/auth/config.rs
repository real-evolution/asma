use serde::Deserialize;
use validator::Validate;

pub const AUTH_CONFIG_SECTION: &str = "auth";

into_fn!(default_validity_seconds: const i64 => 180 * 24 * 60);
into_fn!(default_max_sessions_count: const usize => 8);
into_fn!(default_refresh_token_length: const usize => 128);

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct AuthConfig {
    #[validate(range(min = 1))]
    #[serde(default = "default_validity_seconds")]
    pub signin_validity_seconds: i64,

    #[validate(range(min = 1))]
    #[serde(default = "default_validity_seconds")]
    pub refresh_validity_seconds: i64,

    #[validate(range(min = 1, max = 255))]
    #[serde(default = "default_max_sessions_count")]
    pub max_sessions_count: usize,

    #[validate(range(min = 32, max = 256))]
    #[serde(default = "default_refresh_token_length")]
    pub refresh_token_length: usize,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            signin_validity_seconds: default_validity_seconds(),
            refresh_validity_seconds: default_validity_seconds(),
            max_sessions_count: default_max_sessions_count(),
            refresh_token_length: default_refresh_token_length(),
        }
    }
}
