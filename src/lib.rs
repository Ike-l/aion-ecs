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
pub mod access_result;

pub mod prelude {
    pub use super::{
        world::{
            stored_archetype_storage::{
                StoredArchetypeStorage,
                archetype_id::{
                    ArchetypeId
                },
                archetype_storage::{
                    ArchetypeStorage,
                    ArchetypeStorageRegistry,
                    GLOBAL_ARCHETYPE_STORAGE_CAPACITY,
                    resource_id::{
                        ResourceId
                    },
                    stored_resource_storage::{
                        StoredResourceStorage,
                        resource_storage::{
                            ResourceStorage,
                            ResourceStorageRegistry,
                            entity_storage::{
                                EntityStorage,
                                resource_views::{
                                    unique_resource_view::{
                                        UniqueResourceView
                                    },
                                    shared_resource_view::{
                                        SharedResourceView
                                    },
                                    owned_resource_view::{
                                        NotImplementedOwnedResource
                                    }
                                },
                                entity_map::{
                                    EntityMap,
                                    ITEMS_PER_ENTITY_STORAGE_PAGE,
                                    storage_iter::{
                                        StorageIter
                                    },
                                    storage_iter_mut::{
                                        StorageIterMut
                                    },
                                    stored_resource::{
                                        StoredResource,
                                        resource::{
                                            Resource,
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