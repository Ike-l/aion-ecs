use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, EntityMap, StorageIterMut, StoredResource, UniqueResourceView};

pub mod entity_map;
pub mod resource_views;

pub struct EntityStorage {
    entity_map: EntityMap
}

impl RegistryStorage for EntityStorage {
    type ValueId = EntityId;
    type OwnedValue = StoredResource;
    type ReferencedValue<'a> = UniqueResourceView where Self: 'a;
    type StorageView<'a> = StorageIterMut<'a> where Self: 'a;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        self.entity_map.get_mut(value_id).map(|resource| UniqueResourceView::new(resource))
    }
    
    fn get_view(
        &mut self,
        value_ids: &[&Self::ValueId]
    ) -> Option<Self::StorageView<'_>>
    {
        self.entity_map.iter_mut(value_ids)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        self.entity_map.insert(value_id, value)
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        self.entity_map.remove(value_id)
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.entity_map.valid(value_id)
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