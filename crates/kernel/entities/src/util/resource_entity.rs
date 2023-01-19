use crate::{
    entities::{
        auth::{self, Resource},
        comm,
        link,
    },
    traits::Entity,
};

macro_rules! create_mapping {
    ($entity:ty => $res:expr) => {
        impl ResourceEntity for $entity {
            fn resource() -> Resource {
                $res
            }
        }
    };
}

pub trait ResourceEntity: Entity {
    fn resource() -> Resource;
}

// auth
create_mapping!(auth::User => Resource::User);
create_mapping!(auth::Account => Resource::Account);
create_mapping!(auth::Role => Resource::Role);
create_mapping!(auth::Permission => Resource::Permission);
create_mapping!(auth::Session => Resource::Session);

// link
create_mapping!(link::Channel => Resource::Channel);
create_mapping!(link::Instance => Resource::Instance);
create_mapping!(link::InstanceGroup => Resource::InstanceGroup);

// comm
create_mapping!(comm::Chat => Resource::Chat);
create_mapping!(comm::Message => Resource::Message);
create_mapping!(comm::Bot => Resource::Bot);
create_mapping!(comm::Menu => Resource::Menu);
