use std::{collections::HashMap, sync::Arc};

use aion_state::prelude::Registry;
use parking_lot::RwLock;

use crate::prelude::{EntityId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, ResourceId, EntityStorage, TransmutableOwned, TransmutableShared};


pub mod entity_id;
pub mod entity_storage;

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
    inner: HashMap<ResourceId, EntityStorageRegistry>,
    entities: Vec<EntityId>,
    alive: Arc<RwLock<Vec<bool>>>
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
        archetypes: HashMap<ResourceId, EntityStorageRegistry>
    ) -> Self {
        Self {
            inner: archetypes,
            entities: vec![],
            alive: Arc::new(RwLock::new(vec![]))
        }
    }

    pub fn get_mut(
        &mut self,
        resource_id: &ResourceId,
    ) -> Option<&mut EntityStorageRegistry> {
        self.inner.get_mut(resource_id)
    }

    pub fn insert(
        &mut self,
        _resource_id: ResourceId,
        _resource_storage_registry: EntityStorageRegistry
    ) -> Option<EntityStorageRegistry> {
        panic!("Archetype should never be modified!");
    }

    pub fn remove(
        &mut self,
        _resource_id: &ResourceId
    ) -> Option<EntityStorageRegistry> {
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