use std::{collections::HashMap, str::FromStr};

use enumflags2::{bitflags, BitFlags};
use lazy_static::lazy_static;

use crate::error::AuthError;

lazy_static! {
    static ref RESOURCES_MAP: HashMap<String, AppResource> = vec![
        AppResource::ThisUser,
        AppResource::ThisAccount,
        AppResource::Users,
        AppResource::Accounts,
        AppResource::Roles,
        AppResource::Messages,
        AppResource::MessageTemplates,
        AppResource::Bots,
        AppResource::Peers,
        AppResource::PeerInstances,
        AppResource::PlatformConnections,
    ]
    .into_iter()
    .map(|r| (r.to_string().to_lowercase(), r))
    .collect();
    static ref MODES_MAP: HashMap<String, AccessMode> = vec![
        AccessMode::View,
        AccessMode::Add,
        AccessMode::Modify,
        AccessMode::Remove,
    ]
    .into_iter()
    .map(|m| (m.to_string().to_lowercase(), m))
    .collect();
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum AppResource {
    ThisUser,
    ThisAccount,
    Users,
    Accounts,
    Roles,
    Messages,
    MessageTemplates,
    Bots,
    Peers,
    PeerInstances,
    PlatformConnections,
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum AccessMode {
    View,
    Add,
    Modify,
    Remove,
}

pub struct AppAccess {
    pub resource: AppResource,
    pub mode: BitFlags<AccessMode>,
}

impl ToString for AppAccess {
    fn to_string(&self) -> String {
        format!(
            "{}_{}",
            self.mode.to_string().to_lowercase(),
            self.resource.to_string().to_lowercase()
        )
    }
}

impl FromStr for AppAccess {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mode, resource) = match s.split_once('_') {
            Some(access_tuple) => access_tuple,
            None => return Err(AuthError::InvalidRole(s.to_string())),
        };

        let resource = *match RESOURCES_MAP.get(&resource.to_lowercase()) {
            Some(resource) => resource,
            None => {
                return Err(AuthError::InvalidRole(format!(
                    "invalid resource: {}",
                    resource
                )))
            }
        };

        let mode = *match MODES_MAP.get(&mode.to_lowercase()) {
            Some(mode) => mode,
            None => {
                return Err(AuthError::InvalidRole(format!(
                    "invalid mode: {}",
                    mode
                )))
            }
        };

        Ok(Self {
            resource,
            mode: BitFlags::from_flag(mode),
        })
    }
}
