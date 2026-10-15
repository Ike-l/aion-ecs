use aion_state::prelude::Registry;

use crate::prelude::{ArchetypeId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, QueryIter, Queryable, StoredArchetypeStorage};

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

// additionally could query into chunks
// which produce a tuple of arrays of [EntityId, Bundle[0], .., Bundle[-1]]
// additionally could query by runtime ids

impl World {
    pub fn query<'a, Q: Queryable>(&'a self) -> QueryIter<'a, Q> {
        QueryIter::new(&self.archetypes).unwrap()
    }

    // fn query<'a, Q: Queryable>(&'a self) -> Q::Output<'a> {
        // find which archetype ids are relevant
        // Q -> Included & Excluded ResourceIds
        // let included_resource_ids: Vec<ResourceId>;
        // let excluded_resource_ids: Vec<ResourceId>;
        // let optional_resource_ids: Vec<ResourceId>;
}