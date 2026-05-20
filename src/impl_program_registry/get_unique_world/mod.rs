use std::sync::Arc;

use aion_program::prelude::{ProgramRegistry, Unique};
use tokio::runtime::Runtime;

use crate::prelude::{GetWorldError, World};

pub trait GetUniqueWorld {
    fn get_unique_world(
        self: &Self, 
        runtime: Option<&Runtime>
    ) -> Result<Unique<'_, World>, GetWorldError>;
}

impl GetUniqueWorld for Arc<ProgramRegistry> {
    fn get_unique_world(
        self: &Self, 
        runtime: Option<&Runtime>
    ) -> Result<Unique<'_, World>, GetWorldError> {
        match runtime {
            Some(runtime) => {
                match self.resolve_async::<Unique<World>>(None, vec![]) {
                    Ok(Ok(world)) => Ok(world),
                    Ok(Err(future_world)) => Ok(runtime.block_on(future_world)),
                    Err(err) => Err(GetWorldError::AsyncError(err)),
                }
            },
            None => {
                match self.resolve::<Unique<World>>(None, vec![]) {
                    Ok(world) => Ok(world),
                    Err(err) => Err(GetWorldError::SyncError(err))
                }
            },
        }
    }
}