use std::collections::HashMap;

use aion_state::prelude::RegistryStorage;

use crate::prelude::{EntityId, StoredResource};

pub mod entity_id;
pub mod stored_resource;

pub struct ResourceStorage {
    inner: HashMap<EntityId, StoredResource>,
}

impl RegistryStorage for ResourceStorage {
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
        todo!()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        todo!()
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        todo!()
    }
}