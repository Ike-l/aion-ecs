use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, EntityStorage, StoredResource};

pub mod entity_storage;

pub struct StoredEntityStorage {
    entity_storage: EntityStorage
}

impl RegistryStorage for StoredEntityStorage {
    type ValueId = EntityId;
    type OwnedValue = StoredResource;
    type ReferencedValue<'a> = &'a mut Self::OwnedValue where Self: 'a;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        self.entity_storage.get_mut(value_id)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        self.entity_storage.insert(value_id, value)
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        self.entity_storage.remove(value_id)
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.entity_storage.contains_key(value_id)
    }

    fn len(&self) -> usize {
        self.entity_storage.len()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.entity_storage.keys()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        unsafe { self.entity_storage.next_insert_may_reallocates() }
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        unsafe { self.entity_storage.next_removal_may_reallocates() }
    }
}