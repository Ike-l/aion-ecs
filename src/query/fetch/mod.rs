use crate::prelude::{AccessResult, EntityId, EntityUniqueView, Optional, ResourceId, SharedResource, UniqueResource};

pub trait Fetch {
    type Output<'a>;

    fn include_resource_ids() -> Vec<ResourceId>;
    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>>;
}

impl Fetch for () {
    type Output<'a> = Self;

    fn include_resource_ids() -> Vec<ResourceId> { vec![] }
    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>> { Some(()) }
}

impl<T: 'static> Fetch for &T {
    type Output<'a> = Vec<(EntityId, SharedResource)>;
    
    fn include_resource_ids() -> Vec<ResourceId> {
        vec![ResourceId::from_type::<T>()]            
    }

    fn execute<'a>(
        mut input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>> {
        if input.len() != 1 { return None }

        let (resource_id, result) = input.remove(0);

        if *resource_id != ResourceId::from_type::<T>() { return None };

        match result {
            AccessResult::Shared(result) => {
                return Some(result.into_iter().collect())
            },
            AccessResult::Unique(_) |
            AccessResult::Owned(_) => None,
        }
    }
}

impl<T: 'static> Fetch for &mut T {
    type Output<'a> = Vec<(EntityId, UniqueResource)>;

    fn include_resource_ids() -> Vec<ResourceId> {
        vec![ResourceId::from_type::<T>()]            
    }

    fn execute<'a>(
        mut input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>> {
        if input.len() != 1 { return None }

        let (resource_id, result) = input.remove(0);

        if *resource_id != ResourceId::from_type::<T>() { return None };

        match result {
            AccessResult::Unique(result) => {
                return Some(result.into_iter().collect())
            },
            AccessResult::Shared(_) |
            AccessResult::Owned(_) => None,
        }
    }
}






impl<O: Optional> Fetch for Option<O> {
    type Output<'a> = Option<O>;

    fn include_resource_ids() -> Vec<ResourceId> { vec![] }

    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>>
    {
        todo!()
    }
}






impl<F1: Fetch, F2: Fetch> Fetch for (F1, F2) {
    type Output<'a> = (F1::Output<'a>, F2::Output<'a>);

    fn include_resource_ids() -> Vec<ResourceId> {
        let mut resource_ids = F1::include_resource_ids();
        resource_ids.extend(F2::include_resource_ids().into_iter());

        resource_ids
    }

    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>>
    {
        todo!()
    }
}

impl<F1: Fetch, F2: Fetch, F3: Fetch> Fetch for (F1, F2, F3) {
    type Output<'a> = (F1::Output<'a>, F2::Output<'a>, F3::Output<'a>);

    fn include_resource_ids() -> Vec<ResourceId> {
        let mut resource_ids = F1::include_resource_ids();
        resource_ids.extend(F2::include_resource_ids().into_iter());
        resource_ids.extend(F3::include_resource_ids().into_iter());

        resource_ids
    }

    fn execute<'a>(
        input: Vec<(
            &'a ResourceId, 
            AccessResult<EntityUniqueView<'a>>
        )>
    ) -> Option<Self::Output<'a>>
    {
        todo!()
    }
}