use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, Shared};
use hecs::{Component, Entity, RefMut};

use crate::prelude::{PrepareGetUnique, World};

pub struct GetUnique<'a, T: Component> {
    prepared_get_unique: PrepareGetUnique<T>,
    world: Shared<'a, World>,
}

impl<'a, T: Component> GetUnique<'a, T> {
    pub fn get_unique(&'a self) -> RefMut<'a, T> {
        self.prepared_get_unique.get(&self.world)
    }
}

impl<'a, T: Component> Injection for GetUnique<'a, T> {
    type Item<'new> = GetUnique<'new, T>;

    fn claim_manual_access_builders(_accesses: Vec<&AccessBuilder>) -> Vec<usize> { vec![] }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        Shared::<World>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let world = Shared::<World>::resolve_access(entity, program_registry, derived_results)?;
        let prepared_get_unique = world.prepare_get_unique::<T>(entity.ok_or(ResolveResourceError::Resolving)?).ok_or(ResolveResourceError::Resolving)?;

        Ok(GetUnique {
            prepared_get_unique,
            world
        })
    }
}