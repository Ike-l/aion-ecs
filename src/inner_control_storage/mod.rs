use aion_state::prelude::ControlStorage;

use crate::prelude::ReserverId;

pub struct InnerControlStorage<ResourceId> {
    _p: ResourceId
}

impl<ResourceId> ControlStorage for InnerControlStorage<ResourceId> {
    type Id = ReserverId;

    type ResourceId = ResourceId;

    fn check_owner(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool {
        todo!()
    }

    fn release(
        &mut self,
        resource_id: &Self::ResourceId
    ) -> bool {
        todo!()
    }

    fn own(
        &mut self,
        id: Self::Id,
        resource_id: Self::ResourceId
    ) -> bool {
        todo!()
    }

    fn is_owned(
        &self,
        resource_id: &Self::ResourceId
    ) -> bool {
        todo!()
    }

    fn release_id(
        &mut self,
        id: &Self::Id
    ) -> impl Iterator<Item = Self::ResourceId> {
        todo!()
    }
}