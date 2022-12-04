use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_identifier: String,
    pub agent: String,
    pub last_address: String,
}

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct AccessRule {
    pub role_code: String,
    pub permissions: Vec<(Resource, Actions)>,
}
