use std::marker::PhantomData;

use crate::prelude::{Queryable, ResourceId, Transformer};

/// Transforms query Q by skipping entities satisfying transformer T
pub struct Without<Q, T> { _p: PhantomData<(Q, T)> }

impl<Q: Queryable, T: Transformer> Queryable for Without<Q, T> {
    type Output<'a> = Q::Output<'a> where Self: 'a;

    fn exclude_resource_ids() -> Vec<ResourceId> {
        let mut resource_ids = Q::exclude_resource_ids();

        resource_ids.extend(T::include_resource_ids().into_iter());   

        resource_ids
    }

    fn include_resource_ids() -> Vec<ResourceId> {
        Q::include_resource_ids()
    }
}