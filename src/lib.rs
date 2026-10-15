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

pub mod query;

pub mod prelude {
    pub use super::{
        world::{
            World,
            ArchetypeRegistry,
            stored_archetype_storage::{
                StoredArchetypeStorage,
                archetype_id::{
                    ArchetypeId
                },
                archetype_storage::{
                    ArchetypeStorage,
                    StoredArchetypeRegistry,
                    GLOBAL_ARCHETYPE_STORAGE_CAPACITY,
                    archetype_storage_views::{
                        archetype_storage_unique_view::{
                            ArchetypeStorageUniqueView
                        },
                        archetype_storage_shared_view::{
                            ArchetypeStorageSharedView
                        }
                    },
                    resource_id::{
                        ResourceId
                    },
                    stored_archetype::{
                        StoredArchetype,
                        archetype::{
                            Archetype,
                            EntityStorageRegistry,
                            entity_storage_views::{
                                entity_storage_shared_view::{
                                    EntityStorageSharedView
                                },
                                entity_storage_unique_view::{
                                    EntityStorageUniqueView
                                }
                            },
                            entity_monitor::{
                                EntityMonitor
                            },
                            entity_storage::{
                                EntityStorage,
                                stored_resources::{
                                    unique_stored_resource::{
                                        UniqueStoredResource
                                    },
                                    shared_stored_resource::{
                                        SharedStoredResource
                                    },
                                    owned_resource::{
                                        NotImplementedOwnedResource
                                    }
                                },
                                entity_map::{
                                    EntityMap,
                                    ITEMS_PER_ENTITY_STORAGE_PAGE,
                                    entity_views::{
                                        entity_shared_view::{
                                            EntitySharedView
                                        },
                                        entity_unique_view::{
                                            EntityUniqueView
                                        },
                                    },
                                    stored_resource::{
                                        StoredResource,
                                        resource::{
                                            unique_resource::{
                                                UniqueResource
                                            },
                                            shared_resource::{
                                                SharedResource
                                            }
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
        },
        query::{
            query_iter::{
                QueryIter
            },
            fetch::{
                Fetch
            },
            optional::{
                Optional
            },
            or::{
                Or
            },
            queryable::{
                Queryable,
                transforms::{
                    with::{
                        With
                    },
                    without::{
                        Without
                    }
                }
            },
            transformer::{
                Transformer,
                transforms::{
                    only::{
                        Only
                    }
                }
            },
        }
    };
}