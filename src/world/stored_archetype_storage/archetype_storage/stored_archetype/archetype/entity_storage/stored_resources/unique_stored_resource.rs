use aion_state::prelude::StoreValue;

use crate::prelude::{UniqueResource, SharedStoredResource, TransmutableShared};

pub struct UniqueStoredResource {
    resource: UniqueResource
}

impl UniqueStoredResource {
    pub fn new(resource: UniqueResource) -> Self {
        Self { resource }
    }

    pub fn as_shared<T: 'static>(&self) -> &T {
        unsafe { self.resource.as_shared() }
    }

    pub fn as_unique<T: 'static>(&mut self) -> &mut T {
        unsafe { self.resource.as_unique() }
    }
}

impl StoreValue for UniqueStoredResource {
    type Value = UniqueStoredResource;

    fn store(value: Self::Value) -> Self {
        value
    }

    fn take(self) -> Self::Value {
        self
    }
}

impl<'a> TransmutableShared for UniqueStoredResource {
    type AsShared = SharedStoredResource;

    fn transmute(self) -> Self::AsShared {
        SharedStoredResource::new(self.resource)
    }
}
