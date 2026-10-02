use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, EntityStorage, StoredResource};

pub mod entity_storage;

pub struct StoredEntityStorage {
    entity_storage: EntityStorage
}

impl RegistryStorage for StoredEntityStorage {
    type ValueId = EntityId;
    type Value = StoredResource;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value> {
        todo!()
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value> {
        todo!()
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::Value> {
        todo!()
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        vec![].into_iter()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        todo!()
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        todo!()
    }
}