use aion_state::prelude::{Registry, RegistryAcquireAccess};

use crate::prelude::{AccessResult, ArchetypeId, EntityStorageRegistry, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, Queryable, StoredArchetypeRegistry, StoredArchetypeStorage, StoredResource, UniqueResourceView};

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

impl World {
    fn query<Q: Queryable>(&self) -> Q::Output {
        // find which archetype ids are relevant
    }
}