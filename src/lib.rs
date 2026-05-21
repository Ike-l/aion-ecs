pub mod world;
pub mod injection;
pub mod impl_program_registry;

pub mod prelude {
    pub use super::{
        world::{
            World,
            prepared_get_shared::{
                PreparedGetShared
            },
            prepared_get_unique::{
                PreparedGetUnique
            },
            prepared_query::{
                PreparedQuery
            },
            archetype_tracker::{
                ArchetypeTracker,
                type_tracker::{
                    TypeTracker,
                    type_access::{
                        TypeAccess
                    },
                },
            },
        },
        injection::{
            query::{
                Query
            },
            get_shared::{
                GetShared
            },
            get_unique::{
                GetUnique
            },
            get_owned::{
                GetOwned
            }
        },
        impl_program_registry::{
            get_shared_world::{
                GetSharedWorld
            },
            get_unique_world::{
                GetUniqueWorld
            },
        }
    };
}