use std::{marker::PhantomData, sync::Arc};

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError};
use hecs::{Component, Entity};

use crate::prelude::{GetShared, };

pub struct GetOwned<I, T: Component> {
    pub item: I,
    _f: PhantomData<T>
}

impl<I, T: Component> Injection for GetOwned<I, T> 
    where T: ToOwned<Owned = I>
{
    type Item<'new> = GetOwned<I, T>;

    fn claim_manual_access_builders(accesses: Vec<&AccessBuilder>) -> Vec<usize> { GetShared::<T>::claim_manual_access_builders(accesses) }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        GetShared::<T>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let item = (*GetShared::<T>::resolve_access(entity, program_registry, derived_results)?.get_shared()).to_owned();

        Ok(GetOwned {
            item,
            _f: PhantomData::default()
        })
    }
}