use aion_state::prelude::RegistryStorage;

use crate::prelude::{ResourceId, ResourceStorage, ResourceStorageRegistry};

pub mod resource_storage;

pub struct StoredResourceStorage {
    resource_storage: ResourceStorage
}

impl RegistryStorage for StoredResourceStorage {
    type ValueId = ResourceId;
    
    type OwnedValue = ResourceStorageRegistry;
    type ReferencedValue<'a> = &'a mut Self::OwnedValue where Self: 'a;

    fn get_mut(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        self.resource_storage.get_mut(value_id)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        self.resource_storage.insert(value_id, value)
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        self.resource_storage.remove(value_id)
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.resource_storage.contains_key(value_id)
    }

    fn len(&self) -> usize {
        self.resource_storage.len()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.resource_storage.keys()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        unsafe { self.resource_storage.next_insert_may_reallocates() }
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        unsafe { self.resource_storage.next_removal_may_reallocates() }
    }
}