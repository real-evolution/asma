use serde::Deserialize;
use validator::Validate;

pub const DATA_CONFIG_SECTION: &str = "data";

#[derive(Debug, Deserialize, Validate)]
pub struct SqlxDataConfig<'a> {
    pub driver: &'a str,
    pub host: &'a str,
    pub port: u16,
    pub username: &'a str,
    pub password: &'a str,
}
