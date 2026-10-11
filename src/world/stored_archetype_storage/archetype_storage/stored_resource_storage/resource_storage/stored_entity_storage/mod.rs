use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, EntityStorage, StoredResource, UniqueResourceView};

pub mod entity_storage;
pub mod resource_views;

pub struct StoredEntityStorage {
    entity_storage: EntityStorage
}

impl RegistryStorage for StoredEntityStorage {
    type ValueId = EntityId;
    type OwnedValue = StoredResource;
    type ReferencedValue<'a> = UniqueResourceView where Self: 'a;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        unsafe { self.entity_storage.get_mut(value_id.id) }.map(|resource| UniqueResourceView::new(resource))
    }

    /// We do not return a replaced value
    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        unsafe { self.entity_storage.write(value_id.id, value) };

        None
    }

    /// We do not return the value
    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        unsafe { self.entity_storage.drop_at(value_id.id) };
        
        None
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.entity_storage.is_alive(value_id.id)
    }

    fn len(&self) -> usize {
        unimplemented!()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        unimplemented!();

        #[allow(unreachable_code)]
        std::iter::empty()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        // uses Pages to never reallocate
        false
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        // uses Pages to never reallocate
        false
    }
}