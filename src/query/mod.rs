use std::{any::TypeId, collections::HashMap, sync::Arc};

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, Shared};
use hecs::{Access, PreparedQuery, PreparedQueryBorrow};

use crate::prelude::World;

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

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
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