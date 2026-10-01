use std::{collections::HashMap, hash::Hash};

use aion_state::prelude::ControlStorage;

use crate::prelude::ReserverId;

pub struct InnerControlStorage<ResourceId> {
    inner: HashMap<ResourceId, ReserverId>
}

impl<ResourceId> ControlStorage for InnerControlStorage<ResourceId> 
    where ResourceId: PartialEq + Eq + Hash
{
    type Id = ReserverId;
    type ResourceId = ResourceId;

    fn check_owner(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool {
        self.inner.get(resource_id).is_some_and(|owner| owner == id)
    }

    fn release(
        &mut self,
        resource_id: &Self::ResourceId
    ) -> bool {
        self.inner.remove(resource_id).is_some()
    }

    fn own(
        &mut self,
        id: Self::Id,
        resource_id: Self::ResourceId
    ) -> bool {
        self.inner.insert(resource_id, id);

        true
    }

    fn is_owned(
        &self,
        resource_id: &Self::ResourceId
    ) -> bool {
        self.inner.contains_key(resource_id)
    }

    fn release_id(
        &mut self,
        id: &Self::Id
    ) -> impl Iterator<Item = Self::ResourceId> {
        self.inner.extract_if(move |_, owner| owner == id).map(|(resource_id, _)| resource_id)
    }
}