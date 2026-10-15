use aion_state::prelude::{RegistryAcquireView, RegistryAcquireViewError};

use crate::prelude::{Access, AccessResult, ArchetypeId, ArchetypeRegistry, ArchetypeStorageUniqueView, EntityId, EntityStorageUniqueView, Queryable, ResourceId};

pub struct QueryIter<'a, Q: Queryable + 'a> {
    iterable: Vec<(EntityId, Q::Output<'a>)>
}

impl<'a, Q: Queryable> QueryIter<'a, Q> {
    pub fn new(archetypes: &'a ArchetypeRegistry) -> Result<Self, RegistryAcquireViewError> {
        let include_resource_ids: Vec<ResourceId> = Q::include_resource_ids();
        let exclude_resource_ids: Vec<ResourceId> = Q::exclude_resource_ids();

        let all_archetype_ids = archetypes.keys();

        let archetype_ids: Vec<ArchetypeId> = all_archetype_ids.into_iter().filter(|archetype_id| {
            include_resource_ids.iter().all(|include_resource_id| {
                archetype_id.contains(include_resource_id)
            }) &&
            exclude_resource_ids.iter().all(|exclude_resource_id| {
                !archetype_id.contains(exclude_resource_id)
            })
        }).collect();

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
                            let result = archetype.acquire_view::<AccessResult<EntityStorageUniqueView<'a>>>(RegistryAcquireView { 
                                user_details: None, 
                                resource_details: todo!(), 
                                access: todo!() 
                            });
                        }
                    },
                    AccessResult::Unique(_) |
                    AccessResult::Owned(_) => unreachable!(),
                }
            },
            Err(err) => {
                return Err(err);
            },
        }

        todo!()
    }
}