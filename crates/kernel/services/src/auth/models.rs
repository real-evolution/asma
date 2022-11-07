use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_identifier: String,
    pub agent: String,
    pub last_address: Option<String>,
}
