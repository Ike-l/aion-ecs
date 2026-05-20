use std::sync::Arc;

use aion_program::prelude::{ProgramRegistry, Shared};
use tokio::runtime::Runtime;

use crate::prelude::{GetWorldError, World};

pub trait GetSharedWorld {
    fn get_shared_world(
        self: &Arc<Self>, 
        runtime: Option<&Runtime>
    ) -> Result<Shared<'_, World>, GetWorldError>;
}

impl GetSharedWorld for Arc<ProgramRegistry> {
    fn get_shared_world(
        self: &Arc<Self>, 
        runtime: Option<&Runtime>
    ) -> Result<Shared<'_, World>, GetWorldError> {
        match runtime {
            Some(runtime) => {
                match self.resolve_async::<Shared<World>>(None, vec![]) {
                    Ok(Ok(world)) => Ok(world),
                    Ok(Err(future_world)) => Ok(runtime.block_on(future_world)),
                    Err(err) => Err(GetWorldError::AsyncError(err)),
                }
            },
            None => {
                match self.resolve::<Shared<World>>(None, vec![]) {
                    Ok(world) => Ok(world),
                    Err(err) => Err(GetWorldError::SyncError(err))
                }
            },
        }
    }
}