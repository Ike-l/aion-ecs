use crate::prelude::UniqueResource;

pub struct SharedStoredResource {
    resource: UniqueResource
}

impl SharedStoredResource {
    pub fn new(resource: UniqueResource) -> Self {
        Self { resource }
    }

    pub fn as_shared<T: 'static>(&self) -> &T {
        unsafe { self.resource.as_shared() }
    }
}
