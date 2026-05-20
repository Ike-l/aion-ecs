use aion_program::prelude::{ProgramRegistryResolveAsyncError, ProgramRegistryResolveError};

pub enum GetWorldError {
    SyncError(ProgramRegistryResolveError),
    AsyncError(ProgramRegistryResolveAsyncError)
}