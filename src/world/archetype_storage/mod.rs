use aion_state::prelude::{Registry, RegistryStorage};

use crate::prelude::{InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceId, StoredResourceStorage};

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
    type OwnedValue = StoredResourceStorage;
    type ReferencedValue<'a> = &'a mut Self::OwnedValue where Self: 'a;

    fn get_mut_wrapped(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>>
    {
        todo!()
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        todo!()
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
        todo!()
    }
}