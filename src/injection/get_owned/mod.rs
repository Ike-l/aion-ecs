use std::{marker::PhantomData, sync::Arc};

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, Shared};
use hecs::{Component, Entity};

use crate::prelude::World;

pub struct GetOwned<I, T: Component> {
    pub item: I,
    _f: PhantomData<T>
}

impl<I, T: Component> Injection for GetOwned<I, T> 
    where T: ToOwned<Owned = I>
{
    type Item<'new> = GetOwned<I, T>;

    fn claim_manual_access_builders(_accesses: Vec<&AccessBuilder>) -> Vec<usize> { vec![] }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        Shared::<World>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let world = Shared::<World>::resolve_access(entity, program_registry, derived_results)?;
        let prepared_get_shared = world.prepare_get_shared::<T>(entity.ok_or(ResolveResourceError::Resolving)?).ok_or(ResolveResourceError::Resolving)?;

        let item = (*prepared_get_shared.get(&world)).to_owned();

        Ok(GetOwned {
            item,
            _f: PhantomData::default()
        })
    }
}