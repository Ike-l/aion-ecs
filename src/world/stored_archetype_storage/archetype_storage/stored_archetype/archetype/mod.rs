use std::{collections::HashMap, sync::Arc};

use aion_state::prelude::Registry;
use parking_lot::RwLock;

use crate::prelude::{EntityId, EntityMonitor, EntityStorage, EntityStorageUniqueView, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceId, TransmutableOwned, TransmutableShared};


pub mod entity_id;
pub mod entity_storage;
pub mod entity_monitor;
pub mod entity_storage_views;

pub type EntityStorageRegistry = Registry<
    EntityStorage,
    InnerReservationStorage<EntityId>,
    InnerAccessStorage<EntityId>,
    InnerCredentialStorage,
    InnerWhitelistStorage<EntityId>,
    InnerBlacklistStorage<EntityId>,
    InnerControlStorage<EntityId>
>;

impl<'a> TransmutableShared for &'a mut EntityStorageRegistry {
    type AsShared = &'a EntityStorageRegistry;

    fn transmute(self) -> Self::AsShared {
        self
    }
}

impl<'a> TransmutableOwned for &'a mut EntityStorageRegistry {
    type AsOwned = EntityStorageRegistry;

    fn transmute(self) -> Self::AsOwned {
        unimplemented!()
    }
}

pub struct Archetype {
    inner: HashMap<ResourceId, Arc<EntityStorageRegistry>>,
    _entity_monitor: Arc<RwLock<EntityMonitor>>
}

// When Remove EntityId:
// * Drop data in ALL *inner*
// * Mark as Dead in `alive`

// When Insert Entity:
// * Search `alive` for the first dead entity
// * If found:
// ** Mark as Alive
// ** Bump Epoch of EntityId in `entities`
// * If not found:
// ** Mark as Alive 
// ** Append new EntityId
// * Insert Data
// * Return EntityId

// Optimisation:
// When Remove push index to Stack
// When Insert pop index from Stack



impl Archetype {
    pub fn new_empty(
        archetypes: HashMap<ResourceId, Arc<EntityStorageRegistry>>
    ) -> Self {
        Self {
            inner: archetypes,
            _entity_monitor: Default::default()
        }
    }

    pub fn get_mut(
        &mut self,
        resource_id: &ResourceId,
    ) -> Option<&mut Arc<EntityStorageRegistry>> {
        self.inner.get_mut(resource_id)
    }

    pub fn get_view<'a>(
        &'a mut self,
        resource_ids: &[&ResourceId]
    ) -> Option<EntityStorageUniqueView<'a>> {
        if !resource_ids.iter().all(|resource_id| {
            self.inner.contains_key(*resource_id)
        }) {
            return None
        }

        let entity_storages = self.inner.iter_mut().filter_map(|(resource_id, entity_storage)| {
            if resource_ids.contains(&resource_id) {
                Some((resource_id, entity_storage))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        Some(EntityStorageUniqueView::new(
            entity_storages
        ))
    }

    pub fn insert(
        &mut self,
        _resource_id: ResourceId,
        _resource_storage_registry: Arc<EntityStorageRegistry>
    ) -> Option<Arc<EntityStorageRegistry>> {
        panic!("Archetype should never be modified!");
    }

    pub fn remove(
        &mut self,
        _resource_id: &ResourceId
    ) -> Option<Arc<EntityStorageRegistry>> {
        panic!("Archetype should never be modified!");
    }
    
    pub fn contains_key(
        &self,
        resource_id: &ResourceId
    ) -> bool {
        self.inner.contains_key(resource_id)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &ResourceId> {
        self.inner.keys()
    }

    pub unsafe fn next_insert_may_reallocates(&self) -> bool {
        unreachable!("Archetype should never be modified!");
    }

    pub unsafe fn next_removal_may_reallocates(&self) -> bool {
        unreachable!("Archetype should never be modified!");
    }
}