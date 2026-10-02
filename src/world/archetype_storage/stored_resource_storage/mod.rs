use aion_state::prelude::{RegistryStorage, StoredValueTrait};

use crate::prelude::{ResourceId, ResourceStorage, StoredResource};

pub mod resource_storage;

pub struct StoredResourceStorage {
    resource_storage: ResourceStorage
}

impl StoredValueTrait for StoredResourceStorage {
    type Value = ResourceStorage;

    fn new(value: Self::Value) -> Self {
        Self {
            resource_storage: value
        }
    }

    fn as_shared(&self) -> &Self::Value {
        &self.resource_storage
    }

    fn as_unique(&mut self) -> &mut Self::Value {
        &mut self.resource_storage
    }

    fn into_inner(self) -> Self::Value {
        self.resource_storage
    }
}

impl RegistryStorage for StoredResourceStorage {
    type ValueId = ResourceId;
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