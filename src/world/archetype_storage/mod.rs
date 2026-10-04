use std::{collections::HashMap, sync::OnceLock};

use aion_state::prelude::Registry;

use crate::prelude::{ArchetypeId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceId, StoredResourceStorage};

pub mod stored_resource_storage;
pub mod resource_id;

pub type ArchetypeStorageRegistry = Registry<
    StoredResourceStorage,
    InnerReservationStorage<ResourceId>,
    InnerAccessStorage<ResourceId>,
    InnerCredentialStorage,
    InnerWhitelistStorage<ResourceId>,
    InnerBlacklistStorage<ResourceId>,
    InnerControlStorage<ResourceId>
>;

pub static GLOBAL_ARCHETYPE_STORAGE_CAPACITY: OnceLock<usize> = OnceLock::new();

pub struct ArchetypeStorage {
    inner: HashMap<ArchetypeId, ArchetypeStorageRegistry>,
    calculated_len: usize,
    capacity: usize
}

impl Default for ArchetypeStorage {
    fn default() -> Self {
        let capacity = *GLOBAL_ARCHETYPE_STORAGE_CAPACITY.get_or_init(|| 1000);
        Self::new(capacity)
    }
}

impl ArchetypeStorage {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
            calculated_len: 0,
            capacity
        }
    }
}

impl ArchetypeStorage {
    pub fn get(
        &mut self,
        archetype_id: &ArchetypeId
    ) -> Option<&ArchetypeStorageRegistry> {
        self.inner.get(archetype_id)
    }

    pub fn insert(
        &mut self, 
        archetype_id: ArchetypeId, 
        archetype_storage_registry: ArchetypeStorageRegistry
    ) -> Option<ArchetypeStorageRegistry> {
        let r = self.inner.insert(archetype_id, archetype_storage_registry);

        if r.is_none() {
            self.calculated_len += 1;
        }

        r
    }

    pub fn remove(
        &mut self, 
        archetype_id: &ArchetypeId
    ) -> Option<ArchetypeStorageRegistry> {
        let r = self.inner.remove(archetype_id);

        if r.is_some() {
            self.calculated_len -= 1;
        }

        r
    }

    pub fn contains_key(
        &self, 
        archetype_id: &ArchetypeId
    ) -> bool {
        self.inner.contains_key(archetype_id)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &ArchetypeId> {
        self.inner.keys()
    }

    pub unsafe fn next_insert_may_reallocates(&self) -> bool {
        self.calculated_len >= self.capacity
    }

    pub unsafe fn next_removal_may_reallocates(&self) -> bool {
        false
    }
}