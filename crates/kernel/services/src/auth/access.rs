use derive_more::Display;
use enumflags2::{bitflags, BitFlags};

#[derive(Display)]
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
#[derive(Debug, Clone, Copy, Display)]
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

pub struct RolesIter {
    resource: AppResource,
    mode_iter: enumflags2::Iter<AccessMode>,
}

impl IntoIterator for AppAccess {
    type Item = String;
    type IntoIter = RolesIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            resource: self.resource,
            mode_iter: self.mode.iter(),
        }
    }
}

impl Iterator for RolesIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mode) = self.mode_iter.next() {
            return Some(format!("{}{}", mode, self.resource));
        }

        None
    }
}

impl AppAccess {
    pub fn new(resource: AppResource, mode: BitFlags<AccessMode>) -> Self {
        Self { resource, mode }
    }

    pub fn into_roles_iter(self) -> RolesIter {
        self.into_iter()
    }
}
