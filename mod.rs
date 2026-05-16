use std::{any::TypeId, collections::HashMap};

use hecs::{Access, Bundle, ComponentRef, DynamicBundle, Entity, PreparedQuery, PreparedQueryBorrow, Query};
use parking_lot::RwLock;

use crate::prelude::TrackedAccess;

pub mod tracked_access;

#[derive(Default)]
pub struct World {
    world: hecs::World,
    tracking: RwLock<HashMap<Vec<TypeId>, TrackedAccess>>,
    // tracked_entities: HashMap<Entity, HashMap<TypeId, TrackedAccess>>
}

impl World {
    fn check_query<Q: Query>(&self) -> Option<HashMap<Vec<TypeId>, Access>> {
        let mut new_accesses = HashMap::new();

        let tracking = self.tracking.read();

        let compatible = !self.world.archetypes().any(|archetype| {
            if let Some(access) = archetype.access::<Q>() {
                let component_types = archetype.component_types().collect::<Vec<_>>();
                let compatible = match (access, tracking.get(&component_types)) {
                    (_, None) => true,
                    (_, Some(TrackedAccess::Read(0))) => true,
    
                    (Access::Iterate, _) |
                    (_, Some(TrackedAccess::Iterate)) => true,
    
                    (_, Some(TrackedAccess::Write)) |
                    (Access::Write, _) => false,
    
                    (Access::Read, Some(TrackedAccess::Read(_))) => true,
                };
    
                if compatible {
                    // Safety:
                    // We get the ids directly from archetype
                    let entity_tracking = archetype
                        .ids()
                        .into_iter()
                        .map(|raw_id| {
                            (
                                unsafe { self.world.find_entity_from_id(*raw_id) },
                                access,
                                
                            )
                        });
                    new_accesses.insert(component_types, access);
                }

                !compatible
            } else { true }
        });

        if compatible {
            Some(new_accesses)
        } else {
            None
        }
    }

    fn prepare_query<Q: Query>(&self) -> Option<(PreparedQuery<Q>, HashMap<Vec<TypeId>, Access>)> {
        if let Some(new_accesses) = self.check_query::<Q>() {
            let mut tracking = self.tracking.write();
            let mut prepared_accesses = HashMap::new();
            
            // <<Q as Query>::Fetch as hecs::Fetch>::for_each_borrow(|ty, mut exclusive| {});

            for (types, access) in new_accesses {
                match tracking.get_mut(&types) {
                    Some(current_access) => {
                        match (access, current_access) {
                            (Access::Iterate, _) => (),
                            (Access::Read, TrackedAccess::Read(num)) => { 
                                *num += 1;
                                prepared_accesses.insert(types, access);
                            },
                            (_, TrackedAccess::Write) |
                            (Access::Write, TrackedAccess::Read(_)) => unreachable!(),
                            (access, current_access @ TrackedAccess::Iterate) => {
                                let new_tracked_access = TrackedAccess::from(access);
                                *current_access = new_tracked_access;
                                prepared_accesses.insert(types, access);
                            },
                        }
                    },
                    None => {
                        let new_tracked_access = TrackedAccess::from(access);
                        tracking.insert(types.clone(), new_tracked_access);
                        prepared_accesses.insert(types, access);
                    },
                }
            }

            Some((PreparedQuery::<Q>::new(), prepared_accesses))
        } else { None }
    }

    fn finalise_query(&self, access_ids: &HashMap<Vec<TypeId>, Access>) {
        let mut tracking = self.tracking.write();
        for (types, access) in access_ids {
            match tracking.get_mut(types) {
                Some(current_access) => {
                    match (access, current_access) {
                        (Access::Iterate, _) => (),
                        (Access::Read, TrackedAccess::Read(num)) => *num = if let Some(new_num) = num.checked_sub(1) { new_num } else { 0 },
                        _ => unreachable!()
                    }
                },
                None => todo!(),
            }
        }
    }

    fn do_query<'a, Q: Query>(&'a self, prepared_query: &'a mut PreparedQuery<Q>) -> PreparedQueryBorrow<'a, Q> {
        prepared_query.query(&self.world)
    }

    fn insert(&mut self, entity: Entity, components: impl DynamicBundle) -> Result<(), hecs::NoSuchEntity> {
        self.world.insert(entity, components)
    }

    fn remove<B: Bundle + 'static>(&mut self, entity: Entity) -> Result<B, hecs::ComponentError> {
        self.world.remove::<B>(entity)
    }

    // Just make &mut so cant panic 
    // in future track entities and each typeid's access 
    fn get<'a, T: ComponentRef<'a>>(&'a self, entity: Entity) -> Result<T::Ref, hecs::ComponentError> {
        
        self.world.get::<T>(entity)
    }
}