use chrono::Duration;
use derive_more::Constructor;

#[derive(Constructor, Debug)]
pub struct ChannelInfo {
    pub name: String,
    pub api_key: String,
    pub valid_for: Duration,
}
