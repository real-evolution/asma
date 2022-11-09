use std::{collections::HashMap, str::FromStr};

use convert_case::{Case, Casing};
use enumflags2::{bitflags, BitFlags};
use lazy_static::lazy_static;

use crate::error::AuthError;

lazy_static! {
    static ref AVAILABLE_RESOURCES: Vec<AppResource> = vec![
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
    ];
    static ref RESOURCES_MAP: HashMap<String, AppResource> =
        AVAILABLE_RESOURCES
            .clone()
            .into_iter()
            .map(|r| (r.to_string().to_case(Case::Snake), r))
            .collect();
    static ref MODES_MAP: HashMap<String, AccessMode> = vec![
        AccessMode::View,
        AccessMode::Add,
        AccessMode::Modify,
        AccessMode::Remove,
    ]
    .into_iter()
    .map(|m| (m.to_string().to_case(Case::Snake), m))
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

impl AppResource {
    pub fn get_all() -> &'static Vec<AppResource> {
        &AVAILABLE_RESOURCES
    }
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

#[derive(derive_more::Constructor)]
pub struct AppAccess {
    pub resource: AppResource,
    pub mode: BitFlags<AccessMode>,
}

impl AppAccess {
    pub fn new_full(resource: AppResource) -> Self {
        Self::new(resource, BitFlags::all())
    }

    pub fn into_string_vec(self) -> Vec<String> {
        self.mode.iter().map(|m| self.get_role_string(m)).collect()
    }

    pub fn into_string_map(self) -> HashMap<String, String> {
        self.mode
            .iter()
            .map(|m| (self.get_role_string(m), self.get_friendly_string(m)))
            .collect()
    }

    fn get_friendly_string(&self, mode: AccessMode) -> String {
        format!(
            "Can {} {}",
            mode.to_string().to_lowercase(),
            self.resource
                .to_string()
                .to_case(Case::Title)
                .to_lowercase()
        )
    }

    fn get_role_string(&self, mode: AccessMode) -> String {
        format!(
            "{}__{}",
            mode.to_string().to_case(Case::Snake),
            self.resource.to_string().to_case(Case::Snake)
        )
    }
}

impl FromStr for AppAccess {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mode, resource) = match s.split_once("__") {
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
