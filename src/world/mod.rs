use aion_state::prelude::{Registry, RegistryAcquireAccess};

use crate::prelude::{AccessResult, ArchetypeId, EntityStorageRegistry, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, QueryIter, Queryable, ResourceId, StoredArchetypeRegistry, StoredArchetypeStorage, StoredResource, UniqueResource};

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
    fn query<'a, Q: Queryable>(&'a self) -> QueryIter<'a, Q> {
        QueryIter::new(&self.archetypes).unwrap()
    }

    // fn query<'a, Q: Queryable>(&'a self) -> Q::Output<'a> {
        // find which archetype ids are relevant
        // Q -> Included & Excluded ResourceIds
        // let included_resource_ids: Vec<ResourceId>;
        // let excluded_resource_ids: Vec<ResourceId>;
        // let optional_resource_ids: Vec<ResourceId>;
}