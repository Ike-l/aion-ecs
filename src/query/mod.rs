use std::{any::TypeId, collections::HashMap, sync::Arc};

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, ResourceAccess, ResourceId, Shared};
use hecs::{Access, PreparedQuery, PreparedQueryBorrow};

use crate::prelude::World;

pub const WORLD_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("ECS World");

pub const WORLD_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    program_id: None,
    program_password: None,
    user_details: None,
    resource_id: Some(WORLD_RESOURCE_ID),
    resource_access: Some(ResourceAccess::Shared(1)),
    resource_password: None
};

pub struct Query<'a, Q: hecs::Query> {
    prepared_query: PreparedQuery<Q>,
    access_ids: HashMap<Vec<TypeId>, Access>,
    world: Shared<'a, World>,
}

impl<'a, Q: hecs::Query> Query<'a, Q> {
    pub fn borrow(&mut self) -> PreparedQueryBorrow<'_, Q> {
        self.world.as_ref().do_query(&mut self.prepared_query)
    }
}

impl<'a, Q: hecs::Query> Drop for Query<'a, Q> {
    fn drop(&mut self) {
        self.world.as_ref().finalise_query(&self.access_ids);
    }
}

impl<'a, Q: hecs::Query> Injection for Query<'a, Q> {
    type Item<'new> = Query<'new, Q>;

    fn claim_manual_access_builders(_accesses: Vec<&AccessBuilder>) -> Vec<usize> { vec![] }

    fn submit_access(mut prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        let mut world_access_builder = WORLD_ACCESS_BUILDER;
        if prompted_accesses.len() > 0 {
            let auto_access_builder = prompted_accesses.remove(0);
            world_access_builder = auto_access_builder;
        }

        prompted_accesses.insert(0, world_access_builder);

        Shared::<World>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let world = Shared::<World>::resolve_access(program_registry, derived_results)?;
        let (prepared_query, access_ids) = world.as_ref().prepare_query().ok_or(ResolveResourceError::Resolving)?;
        Ok(Query {
            prepared_query,
            access_ids,
            world
        })
    }
}