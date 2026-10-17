use crate::prelude::{AccessResult, EntityUniqueView, Fetch, ResourceId};

pub mod transforms;

pub trait Queryable {
    type Output<'a> where Self: 'a;

    fn include_resource_ids() -> Vec<ResourceId>;
    fn exclude_resource_ids() -> Vec<ResourceId>;

    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>> where Self: 'a;

    fn transform<'a>(input: &mut Vec<(&'a ResourceId, AccessResult<EntityUniqueView<'a>>)>);
}

impl<T: Fetch> Queryable for T {
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn exclude_resource_ids() -> Vec<ResourceId> { vec![] }

    fn include_resource_ids() -> Vec<ResourceId> {
        <T as Fetch>::include_resource_ids()        
    }

    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>> where T: 'a {
        <T as Fetch>::execute(input)
    }

    fn transform<'a>(_input: &mut Vec<(&'a ResourceId, AccessResult<EntityUniqueView<'a>>)>) {}
}













