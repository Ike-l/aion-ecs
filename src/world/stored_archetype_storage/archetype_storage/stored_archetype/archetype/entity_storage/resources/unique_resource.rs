use aion_state::prelude::StoreValue;

use crate::prelude::{Resource, SharedResource, TransmutableShared};

pub struct UniqueResource {
    resource: Resource
}

impl UniqueResource {
    pub fn new(resource: Resource) -> Self {
        Self { resource }
    }

    pub fn as_shared<T: 'static>(&self) -> &T {
        unsafe { self.resource.as_shared() }
    }

    pub fn as_unique<T: 'static>(&mut self) -> &mut T {
        unsafe { self.resource.as_unique() }
    }
}

impl StoreValue for UniqueResource {
    type Value = UniqueResource;

    fn store(value: Self::Value) -> Self {
        value
    }

    fn take(self) -> Self::Value {
        self
    }
}

impl<'a> TransmutableShared for UniqueResource {
    type AsShared = SharedResource;

    fn transmute(self) -> Self::AsShared {
        SharedResource::new(self.resource)
    }
}
