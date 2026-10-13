use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, EntityMap, StoredResource, UniqueResourceView};

pub mod entity_map;
pub mod resource_views;

pub struct EntityStorage {
    entity_map: EntityMap
}

impl RegistryStorage for EntityStorage {
    type ValueId = EntityId;
    type OwnedValue = StoredResource;
    type ReferencedValue<'a> = UniqueResourceView where Self: 'a;
    type StorageView<'a> = UniqueResourceView where Self: 'a;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        unsafe { self.entity_map.get_mut(value_id.id) }.map(|resource| UniqueResourceView::new(resource))
    }
    
    fn get_view(
        &mut self,
        value_ids: &[&Self::ValueId]
    ) -> Option<Self::StorageView<'_>>
    {
        // self.entity_map.iter_mut(value_ids);
        todo!()
    }

    /// We do not return a replaced value
    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        unsafe { self.entity_map.write(value_id.id, value) };

        None
    }

    /// We do not return the value
    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        unsafe { self.entity_map.drop_at(value_id.id) };
        
        None
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.entity_map.is_alive(value_id.id)
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