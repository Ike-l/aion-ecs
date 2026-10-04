use std::sync::Arc;

use aion_state::prelude::{Registry, RegistryAcquireAccessError, RegistryOwnedAcquireAccess, RegistryReplacement, RegistryStorage, Releaser, ReleasingResult};

use crate::prelude::{AccessResult, EntityId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, Resource, StoredEntityStorage, StoredResource};


pub mod entity_id;
pub mod stored_entity_storage;

pub type ResourceStorageInner = Registry<
    StoredEntityStorage,
    InnerReservationStorage<EntityId>,
    InnerAccessStorage<EntityId>,
    InnerCredentialStorage,
    InnerWhitelistStorage<EntityId>,
    InnerBlacklistStorage<EntityId>,
    InnerControlStorage<EntityId>
>;

pub struct ResourceStorage {
    inner: Arc<ResourceStorageInner>
}

impl ResourceStorage {
    pub fn get(&self) -> Result<ReleasingResult<'_, &mut StoredResource, AccessResult<&mut StoredResource>, ResourceStorageInner>, RegistryAcquireAccessError> {
        ResourceStorageInner::acquire_released_access::<AccessResult<<StoredEntityStorage as RegistryStorage>::ReferencedValue<'_>>>(&self.inner, RegistryOwnedAcquireAccess {
            user_details: todo!(),
            resource_id: todo!(),
            access: todo!(),
            password: todo!(),
        })
    }

    pub fn insert(&mut self) -> Option<Resource> {
        let result = self.inner.checked_replace(RegistryReplacement {
            user_details: todo!(),
            access: todo!(),
            resource_id: todo!(),
            resource: todo!(),
            password: todo!(),
        });

        match result {
            aion_state::prelude::RegistryCheckedReplacementResult::Found(found) => Some(found),
            aion_state::prelude::RegistryCheckedReplacementResult::NotFound => None,
            _ => None,
        }
    }
}