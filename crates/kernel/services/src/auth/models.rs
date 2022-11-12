use derive_more::Constructor;
use kernel_entities::entities::auth::{Resource, Actions};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_identifier: String,
    pub agent: String,
    pub last_address: String,
}

#[derive(Debug, Constructor)]
pub struct AccessRule {
    pub role_code: String,
    pub permissions: Vec<(Resource, Actions)>
}
