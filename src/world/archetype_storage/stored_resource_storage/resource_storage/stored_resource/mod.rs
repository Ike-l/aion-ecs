use aion_state::prelude::StoredValueTrait;

use crate::prelude::Resource;

pub mod resource;

pub struct StoredResource {
    resource: Resource
}

impl StoredValueTrait for StoredResource {
    type Value = Resource;

    fn new(value: Self::Value) -> Self {
        Self {
            resource: value
        }
    }

    fn as_shared(&self) -> &Self::Value {
        &self.resource
    }

    fn as_unique(&mut self) -> &mut Self::Value {
        &mut self.resource
    }

    fn into_inner(self) -> Self::Value {
        self.resource
    }
}