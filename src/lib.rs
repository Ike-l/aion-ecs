// Query<Q, SharedWorld/UniqueWorld
pub mod world;
pub mod query;

pub mod prelude {
    pub use super::{
        world::{
            World,
            tracked_access::{
                TrackedAccess
            }
        },
        query::{
            Query,
            WORLD_RESOURCE_ID,
            SHARED_WORLD_ACCESS_BUILDER,
            UNIQUE_WORLD_ACCESS_BUILDER,
        }
    };
}