use common_validation::custom::*;
use common_validation::parse::*;
use common_validation::validators::traits::ValidateString;

use serde::Deserialize;
use validator::Validate;

pub const DATA_CONFIG_SECTION: &str = "data";
pub const POOL_CONFIG_SECTION: &str = "pool";

#[derive(Debug, Deserialize, Validate)]
pub struct DataConfig<'a> {
    #[validate(custom = "supported_driver")]
    pub driver: &'a str,
    #[validate(custom = "endpoint")]
    pub host: &'a str,
    #[validate(range(min = 0, max = 0xFFFF))]
    pub port: Option<u16>,
    #[validate(custom = "identifier")]
    pub username: &'a str,
    pub password: &'a str,
    #[validate(custom = "identifier")]
    pub database: &'a str,
    #[validate]
    pub pool: PoolConfig,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct PoolConfig {
    #[validate(range(min = 1, max = 64))]
    pub min_connections: Option<u32>,
    #[validate(range(min = 1, max = 64))]
    pub max_connections: Option<u32>,
    pub max_lifetime_ms: Option<u64>,
    pub idle_timeout_ms: Option<u64>,
    pub lazy: Option<bool>,
}
