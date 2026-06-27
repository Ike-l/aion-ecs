use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedError, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, ResolvedResource, Shared};
use anyhow::anyhow;
use hecs::{Entity, QueryBorrow};

use crate::prelude::{World, PreparedQuery};

pub struct Query<'a, Q: hecs::Query> {
    prepared_query: PreparedQuery<Q>,
    world: Shared<'a, World>,
}

impl<'a, Q: hecs::Query> Query<'a, Q> {
    pub fn query(&'a self) -> QueryBorrow<'a, Q> {
        self.prepared_query.query(&self.world)
    }
}

impl<'a, Q: hecs::Query> Injection for Query<'a, Q> {
    type Item<'new> = Query<'new, Q>;

    fn claim_manual_access_builders(accesses: Vec<&AccessBuilder>) -> Vec<usize> { Shared::<World>::claim_manual_access_builders(accesses) }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        Shared::<World>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<Result<ResolvedResource<'new>, DerivedError>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let world = Shared::<World>::resolve_access(entity, program_registry, derived_results)?;
        let prepared_query = world.prepare_query::<Q>().ok_or(
            ResolveResourceError::CanWaitUnknownError(anyhow!("World failed to Prepare"))
        )?;

        Ok(Query {
            prepared_query,
            world
        })
    }
}