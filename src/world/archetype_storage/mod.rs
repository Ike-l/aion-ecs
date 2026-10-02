use aion_state::prelude::{Registry, RegistryAcquireAccess, RegistryStorage};

use crate::prelude::{ArchetypeId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceId, ResourceStorage, StoredResourceStorage};

pub mod stored_resource_storage;
pub mod resource_id;

pub struct ArchetypeStorage {
    inner: Registry<
        StoredResourceStorage,
        InnerReservationStorage<ResourceId>,
        InnerAccessStorage<ResourceId>,
        InnerCredentialStorage,
        InnerWhitelistStorage<ResourceId>,
        InnerBlacklistStorage<ResourceId>,
        InnerControlStorage<ResourceId>
    >
}

impl RegistryStorage for ArchetypeStorage {
    type ValueId = ResourceId;
    type Value = StoredResourceStorage;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value> {
        // let r: Result<_, aion_state::prelude::RegistryAcquireAccessError> = self.inner.acquire_access(RegistryAcquireAccess {
        //     user_details: todo!(),
        //     resource_id: todo!(),
        //     access: todo!(),
        //     password: todo!(),
        // });
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
        todo!()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        todo!()
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        todo!()
    }
}