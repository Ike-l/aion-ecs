use crate::prelude::{Fetch, ResourceId};

pub mod transforms;

pub trait Queryable {
    type Output<'a> where Self: 'a;

    fn include_resource_ids() -> Vec<ResourceId>;
    fn exclude_resource_ids() -> Vec<ResourceId>;

    
}

impl<T: Fetch> Queryable for T {
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn exclude_resource_ids() -> Vec<ResourceId> { vec![] }

    fn include_resource_ids() -> Vec<ResourceId> {
        <T as Fetch>::include_resource_ids()        
    }
}













