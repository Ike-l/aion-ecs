use aion_state::prelude::{Registry, RegistryAcquireAccess};

use crate::prelude::{AccessResult, ArchetypeId, StoredArchetypeRegistry, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, EntityStorageRegistry, StoredArchetypeStorage, StoredResource, UniqueResourceView};

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

// Query:
// find which archetype ids are relevant
// find which resource ids are relevant
// find which entity ids are relevant
// create an iterator over (EntityId, Bundle)
// where a Bundle is a tuple over X
// where an X is an Option<Raw> | Raw
// where a Raw is & | &mut

// additionally could query into chunks
// which produce a tuple of arrays of [EntityId, Bundle[0], .., Bundle[-1]]
// additionally could query by runtime ids

// Filtering
// With
//     Transforms query Q by skipping entities not satisfying query R
// Without
//     Transforms query Q by skipping entities satisfying query R

// Fetching
// Or
//     Holds an L, or an R, or both
// And
//     Holds both a L and R

// Singletons like Time?

impl World {
    fn test(&self) {
        let r = self.archetypes.acquire_access::<AccessResult<&mut StoredArchetypeRegistry>>(RegistryAcquireAccess {
            user_details: todo!(),
            resource_id: todo!(),
            access: todo!(),
            password: todo!(),
        });

        match r {
            Ok(r) => {
                match r {
                    AccessResult::Shared(r) => {
                        let r = r.acquire_access::<AccessResult<&mut EntityStorageRegistry>>(RegistryAcquireAccess {
                            user_details: todo!(),
                            resource_id: todo!(),
                            access: todo!(),
                            password: todo!(),
                        });

                        match r {
                            Ok(r) => {
                                match r {
                                    AccessResult::Shared(r) => {
                                        let r = r.acquire_access::<AccessResult<UniqueResourceView>>(RegistryAcquireAccess {
                                            user_details: todo!(),
                                            resource_id: todo!(),
                                            access: todo!(),
                                            password: todo!(),
                                        });

                                        match r {
                                            Ok(r) => {
                                                match r {
                                                    AccessResult::Shared(r) => {
                                                        // r.as_shared();
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