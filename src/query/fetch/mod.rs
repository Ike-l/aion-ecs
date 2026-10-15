use crate::prelude::{Optional, ResourceId};

pub trait Fetch {
    type Output<'a>;

    fn include_resource_ids() -> Vec<ResourceId>;
}

impl Fetch for () {
    type Output<'a> = Self;

    fn include_resource_ids() -> Vec<ResourceId> { vec![] }
}

impl<T: 'static> Fetch for &T {
    type Output<'a> = &'a T;
    
    fn include_resource_ids() -> Vec<ResourceId> {
        vec![ResourceId::from_type::<T>()]            
    }
}

impl<T: 'static> Fetch for &mut T {
    type Output<'a> = &'a mut T;

    fn include_resource_ids() -> Vec<ResourceId> {
        vec![ResourceId::from_type::<T>()]            
    }
}






impl<O: Optional> Fetch for Option<O> {
    type Output<'a> = Option<O>;

    fn include_resource_ids() -> Vec<ResourceId> { vec![] }
}






impl<F1: Fetch, F2: Fetch> Fetch for (F1, F2) {
    type Output<'a> = (F1::Output<'a>, F2::Output<'a>);

    fn include_resource_ids() -> Vec<ResourceId> {
        let mut resource_ids = F1::include_resource_ids();
        resource_ids.extend(F2::include_resource_ids().into_iter());

        resource_ids
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
}