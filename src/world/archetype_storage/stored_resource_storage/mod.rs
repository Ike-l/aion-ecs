use aion_state::prelude::{RegistryStorage};

use crate::prelude::{AccessResult, Resource, ResourceId, ResourceStorage, StoredResourceStorageOutput};

pub mod resource_storage;
pub mod stored_resource_storage_output;

pub struct StoredResourceStorage {
    resource_storage: ResourceStorage
}

impl RegistryStorage for StoredResourceStorage {
    type ValueId = ResourceId;
    
    type OwnedValue = Resource;
    type ReferencedValue<'a> = StoredResourceStorageOutput<'a> where Self: 'a;

    fn get_mut_wrapped(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>> {
        self.resource_storage.get().ok().map(Self::ReferencedValue::new)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        self.resource_storage.insert()
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
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
        let result = self.resource_storage.get();
        match result {
            Ok(result) => {
                let result = result.as_ref().unwrap();
                match result {
                    AccessResult::Shared(_) => todo!(),
                    _ => unreachable!()
                }
                todo!()
            },
            Err(err) => todo!(),
        }
    }
}