use std::{collections::HashMap, sync::OnceLock};

use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, StoredResource};

pub mod stored_resource;

pub static GLOBAL_CAPACITY: OnceLock<usize> = OnceLock::new();

pub struct EntityStorage {
    inner: HashMap<EntityId, StoredResource>,
    calculated_len: usize,
    capacity: usize
}

impl Default for EntityStorage {
    fn default() -> Self {
        let capacity = *GLOBAL_CAPACITY.get_or_init(|| 1000);
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

impl RegistryStorage for EntityStorage {
    type ValueId = EntityId;
    type Value = StoredResource;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value> {
        self.inner.get_mut(value_id)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value> {
        let r = self.inner.insert(value_id, value);

        if r.is_none() {
            self.calculated_len += 1;
        }

        r
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::Value> {
        let r = self.inner.remove(value_id);

        if r.is_some() {
            self.calculated_len -= 1;
        }

        r
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.inner.contains_key(value_id)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.inner.keys()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        self.calculated_len >= self.capacity
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        false
    }
}