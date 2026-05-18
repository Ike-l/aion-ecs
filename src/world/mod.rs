use std::{any::TypeId, sync::Arc};

use hecs::{Bundle, Component, ComponentError, ComponentRef, DynamicBundle, Entity, Query};
use parking_lot::Mutex;

use crate::prelude::{ArchetypeTracker, PrepareGetShared, PrepareGetUnique, PreparedQuery, TypeAccess};

pub mod prepare_get_shared;
pub mod prepare_get_unique;
pub mod archetype_tracker;
pub mod prepared_query;

#[derive(Default)]
pub struct World {
    hecs_world: hecs::World,
    tracker: Arc<Mutex<ArchetypeTracker>>
}

impl World {
    /// # Safety
    /// 
    /// Ensure Accesses are tracked
    /// 
    /// Use `prepare_query`
    pub(crate) unsafe fn query<Q: Query>(&self) -> hecs::QueryBorrow<'_, Q> {
        self.hecs_world.query::<Q>()
    }

    pub fn prepare_query<Q: Query>(&self) -> Option<PreparedQuery<Q>> {
        let mut tracker = self.tracker.lock();
        let mut query_tracker = ArchetypeTracker::default();

        let mut conflict = false;
        for archetype in self.hecs_world.archetypes() {
            if conflict { continue; }
            if archetype.access::<Q>().is_some() {
                if conflict { continue; }
                <<Q as Query>::Fetch as hecs::Fetch>::for_each_borrow(|type_id, unique| {
                    if conflict { return; }
                    if archetype.has_dynamic(type_id) {
                        let archetype_signature = archetype.component_types().collect::<Vec<_>>();
                        let type_access = if unique { TypeAccess::Unique } else { TypeAccess::Shared(1) };
                        
                        let ok = tracker.try_access(archetype_signature, type_id, type_access);
                        
                        if ok {
                            let archetype_signature = archetype.component_types().collect::<Vec<_>>();
                            let type_access = if unique { TypeAccess::Unique } else { TypeAccess::Shared(1) };

                            query_tracker.try_access(archetype_signature, type_id, type_access);
                        } else {
                            conflict = true;
                        }
                    }
                });
            }
        }

        if conflict {
            tracker.split(&mut query_tracker);
            None
        } else {
            Some(PreparedQuery::new(Arc::clone(&self.tracker), query_tracker))
        }
    }

    fn prepare_get<'a, T: Component>(&'a self, entity: Entity, type_access: TypeAccess) -> Option<ArchetypeTracker> {
        let target_archetypes = self.hecs_world.archetypes().find(|archetype| {
            archetype.ids().contains(&entity.id())
        })?;

        let type_id = TypeId::of::<T>();
        
        let archetype_id = target_archetypes.component_types().collect::<Vec<_>>();

        let mut tracker = self.tracker.lock();
        
        
        let ok = tracker.try_access(archetype_id, type_id, type_access.clone());
        
        if !ok {
            return None;
        }

        let mut get_tracker = ArchetypeTracker::default();
        let archetype_id = target_archetypes.component_types().collect::<Vec<_>>();
        get_tracker.try_access(archetype_id, type_id, type_access);

        Some(get_tracker)
        
    }
    
    pub fn prepare_get_shared<'a, T: Component>(&'a self, entity: Entity) -> Option<PrepareGetShared<T>> {
        let type_access = TypeAccess::Shared(1);
        let get_tracker = self.prepare_get::<T>(entity, type_access)?;
        Some(PrepareGetShared::new(Arc::clone(&self.tracker), get_tracker, entity))
    }

    pub fn prepare_get_unique<'a, T: Component>(&'a self, entity: Entity) -> Option<PrepareGetUnique<T>> {
        let type_access = TypeAccess::Unique;
        let get_tracker = self.prepare_get::<T>(entity, type_access)?;
        Some(PrepareGetUnique::new(Arc::clone(&self.tracker), get_tracker, entity))
    }

    pub(crate) unsafe fn get<'a, T: ComponentRef<'a>>(&'a self, entity: Entity) -> Result<<T as ComponentRef<'a>>::Ref, ComponentError> {
        self.hecs_world.get::<T>(entity)
    }

    pub fn insert(&mut self, entity: Entity, components: impl DynamicBundle) -> Result<(), hecs::NoSuchEntity> {
        self.hecs_world.insert(entity, components)
    }

    pub fn remove<B: Bundle + 'static>(&mut self, entity: Entity) -> Result<B, hecs::ComponentError> {
        self.hecs_world.remove::<B>(entity)
    }    

    pub fn has<T: Component>(&self, entity: Entity) -> Option<bool> {
        Some(self.hecs_world.entity(entity).ok()?.has::<T>())
    }
}