use aion_state::prelude::RegistryStorage;

use crate::prelude::{ArchetypeId, ArchetypeStorage, ArchetypeStorageUniqueView, StoredArchetypeRegistry};

pub mod archetype_storage;
pub mod archetype_id;

pub struct StoredArchetypeStorage {
    archetype_storage: ArchetypeStorage
}

impl RegistryStorage for StoredArchetypeStorage {
    type ValueId = ArchetypeId;
    type OwnedValue = StoredArchetypeRegistry;
    type ReferencedValue<'a> = &'a mut Self::OwnedValue where Self: 'a;
    type StorageView<'a> = ArchetypeStorageUniqueView<'a> where Self: 'a;

    fn get_mut(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        self.archetype_storage.get_mut(value_id)
    }

    fn get_view(
        &mut self,
        value_ids: &[&Self::ValueId]
    ) -> Option<Self::StorageView<'_>>
    {
        self.archetype_storage.get_view(value_ids)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        self.archetype_storage.insert(value_id, value)
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        self.archetype_storage.remove(value_id)
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.archetype_storage.contains_key(value_id)
    }

    fn len(&self) -> usize {
        self.archetype_storage.len()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.archetype_storage.keys()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        unsafe { self.archetype_storage.next_insert_may_reallocates() }
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        unsafe { self.archetype_storage.next_removal_may_reallocates() }
    }
}