use crate::prelude::Resource;

pub struct SharedResourceView {
    resource: Resource
}

impl SharedResourceView {
    pub fn new(resource: Resource) -> Self {
        Self { resource }
    }

    pub fn as_shared<T: 'static>(&self) -> &T {
        unsafe { self.resource.as_shared() }
    }
}
