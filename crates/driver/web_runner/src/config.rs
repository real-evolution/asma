use common_validation::custom::*;

use serde::Deserialize;
use validator::Validate;

pub const WEB_CONFIG_SECTION: &str = "web";
pub const WEB_DEFAULT_PORT: u16 = 3434;

#[derive(Debug, Deserialize, Validate)]
pub struct WebConfig {
    #[validate(custom = "ip_endpoint")]
    pub listen_addr: String,

    #[validate(range(min = 0, max = 0xFFFF))]
    pub listen_port: Option<u16>,
}
