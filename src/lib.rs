pub mod world;

pub mod inner_reservation_storage;
pub mod inner_access_storage;
pub mod inner_credential_storage;
pub mod inner_blacklist_storage;
pub mod inner_whitelist_storage;
pub mod inner_control_storage;

pub mod reserver_id;
pub mod password;
pub mod access;

pub mod prelude {
    pub use super::{
        world::{
            World,
            archetype_id::{
                ArchetypeId
            },
            archetype_storage::{
                ArchetypeStorage,
                resource_id::{
                    ResourceId
                },
                stored_resource_storage::{
                    StoredResourceStorage,
                    resource_storage::{
                        ResourceStorage,
                        stored_entity_storage::{
                            StoredEntityStorage,
                            entity_storage::{
                                EntityStorage,
                                GLOBAL_CAPACITY,
                                stored_resource::{
                                    StoredResource,
                                    resource::{
                                        Resource
                                    }
                                },
                            }
                        },
                        entity_id::{
                            EntityId
                        },
                    }
                }
            }
        },
        inner_reservation_storage::{
            InnerReservationStorage
        },
        inner_access_storage::{
            InnerAccessStorage
        },
        inner_credential_storage::{
            InnerCredentialStorage
        },
        inner_blacklist_storage::{
            InnerBlacklistStorage
        },
        inner_whitelist_storage::{
            InnerWhitelistStorage
        },
        inner_control_storage::{
            InnerControlStorage
        },
        reserver_id::{
            ReserverId
        },
        access::{
            Access
        },
        access_result::{
            AccessResult,
            transmutable::{
                TransmutableShared,
                TransmutableOwned
            }
        },
        password::{
            Password
        }
    };
}