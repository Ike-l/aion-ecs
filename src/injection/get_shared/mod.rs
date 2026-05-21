use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, Shared};
use hecs::{Component, Entity, Ref};

use crate::prelude::{PreparedGetShared, World};

pub struct GetShared<'a, T: Component> {
    prepared_get_shared: PreparedGetShared<T>,
    world: Shared<'a, World>,
}

impl<'a, T: Component> GetShared<'a, T> {
    pub fn get_shared(&'a self) -> Ref<'a, T> {
        self.prepared_get_shared.get(&self.world)
    }
}

impl<'a, T: Component> Injection for GetShared<'a, T> {
    type Item<'new> = GetShared<'new, T>;

    fn claim_manual_access_builders(accesses: Vec<&AccessBuilder>) -> Vec<usize> { Shared::<World>::claim_manual_access_builders(accesses) }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        Shared::<World>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let world = Shared::<World>::resolve_access(entity, program_registry, derived_results)?;
        let prepared_get_shared = world.prepare_get_shared::<T>(entity.ok_or(ResolveResourceError::Resolving)?).ok_or(ResolveResourceError::Resolving)?;

        Ok(GetShared {
            prepared_get_shared,
            world
        })
    }
}