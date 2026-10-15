use std::marker::PhantomData;

use crate::prelude::{ResourceId, Transformer};

pub struct Only<T> { _p: PhantomData<T> }

impl<T: 'static> Transformer for Only<T> {
    fn include_resource_ids() -> Vec<ResourceId> {
        vec![ResourceId::from_type::<T>()]
    }
}