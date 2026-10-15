use aion_state::prelude::{RegistryAcquireView, RegistryAcquireViewError};

use crate::prelude::{Access, AccessResult, ArchetypeId, ArchetypeRegistry, ArchetypeStorageUniqueView, EntityId, EntityStorageUniqueView, EntityUniqueView, Queryable, ResourceId};

pub struct QueryIter<'a, Q: Queryable + 'a> {
    iterable: Vec<(EntityId, Q::Output<'a>)>
}

impl<'a, Q: Queryable> QueryIter<'a, Q> {

    pub fn get_relevant_archetype_ids(superset: Vec<ArchetypeId>) -> Vec<ArchetypeId> {
        let include_resource_ids: Vec<ResourceId> = Q::include_resource_ids();
        let exclude_resource_ids: Vec<ResourceId> = Q::exclude_resource_ids();
    
        superset.into_iter().filter(|archetype_id| {
            include_resource_ids.iter().all(|include_resource_id| {
                archetype_id.contains(include_resource_id)
            }) &&
            exclude_resource_ids.iter().all(|exclude_resource_id| {
                !archetype_id.contains(exclude_resource_id)
            })
        }).collect()
    }

    pub fn new(archetypes: &'a ArchetypeRegistry) -> Result<Self, RegistryAcquireViewError> {
        let archetype_ids = Self::get_relevant_archetype_ids(archetypes.keys());

        let resource_details = archetype_ids.into_iter().map(|archetype_id| (archetype_id, None)).collect::<Vec<_>>();

        let result = archetypes.acquire_view::<AccessResult<ArchetypeStorageUniqueView<'a>>>(RegistryAcquireView {
            user_details: None,
            resource_details,
            access: Access::Shared(1),
        });

        match result {
            Ok(result) => {
                match result {
                    AccessResult::Shared(result) => {
                        for (archetype_id, archetype) in result.into_iter() {
                            let resource_ids = Vec::new();
                            let result = archetype.acquire_view::<AccessResult<EntityStorageUniqueView<'a>>>(RegistryAcquireView { 
                                user_details: None, 
                                resource_details: resource_ids, 
                                access: Access::Shared(1) 
                            });

                            match result {
                                Ok(result) => {
                                    match result {
                                        AccessResult::Shared(result) => {
                                            for (resource_id, entity_map) in result.into_iter() {
                                                let entities = entity_map.keys();
                                                let resource_details = entities.into_iter().map(|entity| (entity, None)).collect();
                                                let result = entity_map.acquire_view::<AccessResult<EntityUniqueView<'a>>>(RegistryAcquireView {
                                                    user_details: None,
                                                    resource_details,
                                                    access: todo!()
                                                });

                                                match result {
                                                    Ok(result) => {
                                                        match result {
                                                            AccessResult::Shared(result) => {

                                                                // for (entity_id, resource) in result.into_iter() {
                                                                //     // resource.as_shared()
                                                                // }
                                                            },
                                                            AccessResult::Unique(result) => {
                                                                for (entity_id, resource) in result.into_iter() {
                                                                    // resource.as_unique()
                                                                }
                                                            },
                                                            AccessResult::Owned(_) => todo!(),
                                                        }
                                                    },
                                                    Err(_) => todo!(),
                                                }
                                            }
                                        },
                                        AccessResult::Unique(_) |
                                        AccessResult::Owned(_) => todo!(),
                                    }
                                },
                                Err(_) => todo!(),
                            }
                            
                        }
                    },
                    AccessResult::Unique(_) |
                    AccessResult::Owned(_) => todo!(),
                }
            },
            Err(_) => todo!(),
        }

        // match result {
        //     Ok(result) => {
        //         match result {
        //             AccessResult::Shared(result) => {
        //                 for (archetype_id, archetype) in result.into_iter() {

        //                     match result {
        //                         Ok(result) => {
        //                             match result {
        //                                 AccessResult::Shared(result) => {
        //                                     for (resource_id, entity_map) in result.into_iter() {

        //                                     }
        //                                 },
        //                                 AccessResult::Unique(_) |
        //                                 AccessResult::Owned(_) => unreachable!(),
        //                             }
        //                         },
        //                         Err(err) => todo!(),
        //                     }
        //                 }
        //             },
        //             AccessResult::Unique(_) |
        //             AccessResult::Owned(_) => unreachable!(),
        //         }
        //     },
        //     Err(err) => todo!(),
        // }

        todo!()
    }
}