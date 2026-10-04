use std::{collections::HashMap, sync::OnceLock};

use crate::prelude::{EntityId, StoredResource};

pub mod stored_resource;

pub static GLOBAL_ENTITY_STORAGE_CAPACITY: OnceLock<usize> = OnceLock::new();

pub struct EntityStorage {
    inner: HashMap<EntityId, StoredResource>,
    calculated_len: usize,
    capacity: usize
}

impl Default for EntityStorage {
    fn default() -> Self {
        let capacity = *GLOBAL_ENTITY_STORAGE_CAPACITY.get_or_init(|| 1000);
        Self::new(capacity)
    }
}

impl EntityStorage {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
            calculated_len: 0,
            capacity
        }
    }
}

impl EntityStorage {
    pub fn get(
        &mut self,
        entity_id: &EntityId
    ) -> Option<&mut StoredResource> {
        self.inner.get_mut(entity_id)
    }

    pub fn insert(
        &mut self, 
        entity_id: EntityId, 
        stored_resource: StoredResource
    ) -> Option<StoredResource> {
        let r = self.inner.insert(entity_id, stored_resource);

        if r.is_none() {
            self.calculated_len += 1;
        }

        r
    }

    pub fn remove(
        &mut self, 
        entity_id: &EntityId
    ) -> Option<StoredResource> {
        let r = self.inner.remove(entity_id);

        if r.is_some() {
            self.calculated_len -= 1;
        }

        r
    }

    pub fn contains_key(
        &self, 
        entity_id: &EntityId
    ) -> bool {
        self.inner.contains_key(entity_id)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &EntityId> {
        self.inner.keys()
    }

    pub unsafe fn next_insert_may_reallocates(&self) -> bool {
        self.calculated_len >= self.capacity
    }

    pub unsafe fn next_removal_may_reallocates(&self) -> bool {
        false
    }
}