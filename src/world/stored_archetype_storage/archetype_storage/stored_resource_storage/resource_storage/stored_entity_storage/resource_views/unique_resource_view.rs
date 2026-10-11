use aion_state::prelude::StoreValue;

use crate::prelude::{Resource, SharedResourceView, TransmutableShared};

pub struct UniqueResourceView {
    resource: Resource
}

impl UniqueResourceView {
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

impl StoreValue for UniqueResourceView {
    type Value = UniqueResourceView;

    fn store(value: Self::Value) -> Self {
        value
    }

    fn take(self) -> Self::Value {
        self
    }
}

impl<'a> TransmutableShared for UniqueResourceView {
    type AsShared = SharedResourceView;

    fn transmute(self) -> Self::AsShared {
        SharedResourceView::new(self.resource)
    }
}
