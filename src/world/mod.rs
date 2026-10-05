use aion_state::prelude::{Registry, RegistryAcquireAccess};

use crate::prelude::{AccessResult, ArchetypeId, ArchetypeStorageRegistry, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceStorageRegistry, StoredArchetypeStorage, StoredResource};

pub mod stored_archetype_storage;

pub type ArchetypeRegistry = Registry<
    StoredArchetypeStorage,
    InnerReservationStorage<ArchetypeId>,
    InnerAccessStorage<ArchetypeId>,
    InnerCredentialStorage,
    InnerWhitelistStorage<ArchetypeId>,
    InnerBlacklistStorage<ArchetypeId>,
    InnerControlStorage<ArchetypeId>
>;

pub struct World {
    archetypes: ArchetypeRegistry
}

impl World {
    fn test(&self) {
        let r = self.archetypes.acquire_access::<AccessResult<&mut ArchetypeStorageRegistry>>(RegistryAcquireAccess {
            user_details: todo!(),
            resource_id: todo!(),
            access: todo!(),
            password: todo!(),
        });

        match r {
            Ok(r) => {
                match r {
                    AccessResult::Shared(r) => {
                        let r = r.acquire_access::<AccessResult<&mut ResourceStorageRegistry>>(RegistryAcquireAccess {
                            user_details: todo!(),
                            resource_id: todo!(),
                            access: todo!(),
                            password: todo!(),
                        });

                        match r {
                            Ok(r) => {
                                match r {
                                    AccessResult::Shared(r) => {
                                        let r = r.acquire_access::<AccessResult<&mut StoredResource>>(RegistryAcquireAccess {
                                            user_details: todo!(),
                                            resource_id: todo!(),
                                            access: todo!(),
                                            password: todo!(),
                                        });

                                        match r {
                                            Ok(r) => {
                                                match r {
                                                    AccessResult::Shared(r) => {
                                                           
                                                    },
                                                    AccessResult::Unique(r) => todo!(),
                                                    AccessResult::Owned(r) => todo!(),
                                                }
                                            },
                                            Err(err) => todo!(),
                                        }
                                    },
                                    AccessResult::Unique(r) => todo!(),
                                    AccessResult::Owned(r) => todo!(),
                                }
                            },
                            Err(err) => todo!(),
                        }
                    },
                    AccessResult::Unique(r) => todo!(),
                    AccessResult::Owned(r) => todo!(),
                }
            },
            Err(err) => todo!(),
        }
    }
}