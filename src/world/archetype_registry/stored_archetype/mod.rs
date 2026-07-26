use std::sync::Arc;

use aion_program::prelude::StoredResourceTrait;

use crate::prelude::Archetype;

pub mod archetype;

pub struct StoredArchetype { 
    inner: Arc<Archetype>
}

impl StoredResourceTrait for StoredArchetype {
    type Resource = Archetype;

    fn new(resource: Self::Resource) -> Self {
        Self { inner: Arc::new(resource) }
    }

    fn get(&self) -> &Self::Resource {
        &self.inner
    }

    fn get_mut(&mut self) -> &mut Self::Resource {
        unreachable!()
    }
}