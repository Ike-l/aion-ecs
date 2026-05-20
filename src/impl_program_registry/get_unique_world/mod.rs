use std::sync::Arc;

use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError, Unique};
use tokio::runtime::Runtime;

use crate::prelude::World;

pub trait GetUniqueWorld {
    fn get_unique_world(
        self: &Self, 
        runtime: Option<&Runtime>
    ) -> Result<Unique<'_, World>, ProgramRegistryResolveEitherError>;
}

impl GetUniqueWorld for Arc<ProgramRegistry> {
    fn get_unique_world(
        self: &Self, 
        runtime: Option<&Runtime>
    ) -> Result<Unique<'_, World>, ProgramRegistryResolveEitherError> {
        self.resolve_simple_either::<Unique<World>>(runtime)
    }
}