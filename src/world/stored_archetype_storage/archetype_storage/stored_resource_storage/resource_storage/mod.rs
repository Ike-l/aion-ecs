use std::{collections::HashMap, sync::{Arc, OnceLock}};

use aion_state::prelude::Registry;

use crate::prelude::{EntityId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceId, StoredEntityStorage, TransmutableOwned, TransmutableShared};


pub mod entity_id;
pub mod stored_entity_storage;

pub type ResourceStorageRegistry = Arc<Registry<
    StoredEntityStorage,
    InnerReservationStorage<EntityId>,
    InnerAccessStorage<EntityId>,
    InnerCredentialStorage,
    InnerWhitelistStorage<EntityId>,
    InnerBlacklistStorage<EntityId>,
    InnerControlStorage<EntityId>
>>;

impl<'a> TransmutableShared for &'a mut ResourceStorageRegistry {
    type AsShared = &'a ResourceStorageRegistry;

    fn transmute(self) -> Self::AsShared {
        self
    }
}

impl<'a> TransmutableOwned for &'a mut ResourceStorageRegistry {
    type AsOwned = ResourceStorageRegistry;

    fn transmute(self) -> Self::AsOwned {
        unimplemented!()
    }
}

pub static GLOBAL_RESOURCE_STORAGE_CAPACITY: OnceLock<usize> = OnceLock::new();

pub struct ResourceStorage {
    inner: HashMap<ResourceId, ResourceStorageRegistry>,
    calculated_len: usize,
    capacity: usize
}

impl Default for ResourceStorage {
    fn default() -> Self {
        let capacity = *GLOBAL_RESOURCE_STORAGE_CAPACITY.get_or_init(|| 1000);
        Self::new(capacity)
    }
}

impl ResourceStorage {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
            calculated_len: 0,
            capacity
        }
    }
}

impl ResourceStorage {
    pub fn get_mut(
        &mut self,
        resource_id: &ResourceId,
    ) -> Option<&mut ResourceStorageRegistry> {
        self.inner.get_mut(resource_id)
    }

    pub fn insert(
        &mut self,
        resource_id: ResourceId,
        resource_storage_registry: ResourceStorageRegistry
    ) -> Option<ResourceStorageRegistry> {
        let r = self.inner.insert(resource_id, resource_storage_registry);

        if r.is_none() {
            self.calculated_len += 1;
        }

        r
    }

    pub fn remove(
        &mut self,
        resource_id: &ResourceId
    ) -> Option<ResourceStorageRegistry> {
        let r = self.inner.remove(resource_id);

        if r.is_some() {
            self.calculated_len -= 1;
        }

        r
    }
    
    pub fn contains_key(
        &self,
        resource_id: &ResourceId
    ) -> bool {
        self.inner.contains_key(resource_id)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &ResourceId> {
        self.inner.keys()
    }

    pub unsafe fn next_insert_may_reallocates(&self) -> bool {
        self.calculated_len >= self.capacity
    }

    pub unsafe fn next_removal_may_reallocates(&self) -> bool {
        false
    }
}