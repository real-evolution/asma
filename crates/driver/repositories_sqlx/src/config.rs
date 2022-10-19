use common_validation::validators::*;

use serde::Deserialize;
use validator::Validate;

pub const DATA_CONFIG_SECTION: &str = "data";

#[derive(Debug, Deserialize, Validate)]
pub struct SqlxDataConfig<'a> {
    #[validate(custom = "supported_driver")]
    pub driver: &'a str,
    #[validate(custom = "endpoint")]
    pub host: &'a str,
    #[validate(range(min = 0, max = 0xFFFF))]
    pub port: Option<u16>,
    #[validate(custom = "basic_username")]
    pub username: &'a str,
    pub password: &'a str,
}
