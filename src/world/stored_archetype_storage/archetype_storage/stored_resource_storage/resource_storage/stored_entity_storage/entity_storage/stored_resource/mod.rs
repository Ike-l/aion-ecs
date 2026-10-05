use aion_state::prelude::StoreValue;

use crate::prelude::{Resource, TransmutableOwned, TransmutableShared};

pub mod resource;

pub struct StoredResource {
    resource: Resource
}

impl StoreValue for StoredResource {
    type Value = Resource;

    fn store(value: Self::Value) -> Self {
        Self {
            resource: value
        }
    }

    fn take(self) -> Self::Value {
        self.resource
    }
}

impl<'a> TransmutableShared for &'a mut StoredResource {
    type AsShared = &'a StoredResource;

    fn transmute(self) -> Self::AsShared { self }
}

impl<'a> TransmutableOwned for &'a mut StoredResource {
    type AsOwned = StoredResource;

    fn transmute(self) -> Self::AsOwned { unimplemented!() }
}