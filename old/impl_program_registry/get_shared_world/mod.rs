use std::sync::Arc;

use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError, Shared};
use tokio::runtime::Runtime;

use crate::prelude::World;

pub trait GetSharedWorld {
    fn get_shared_world(
        self: &Self, 
        runtime: Option<&Runtime>
    ) -> Result<Shared<'_, World>, ProgramRegistryResolveEitherError>;
}

impl GetSharedWorld for Arc<ProgramRegistry> {
    fn get_shared_world(
        self: &Self, 
        runtime: Option<&Runtime>
    ) -> Result<Shared<'_, World>, ProgramRegistryResolveEitherError> {
        self.resolve_simple_either::<Shared<World>>(runtime)
    }
}