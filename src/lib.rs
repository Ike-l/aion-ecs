pub mod world;
pub mod injection;

pub mod prelude {
    pub use super::{
        world::{
            World,
            prepare_get_shared::{
                PrepareGetShared
            },
            prepare_get_unique::{
                PrepareGetUnique
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
            }
        }
    };
}