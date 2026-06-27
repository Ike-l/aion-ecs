pub mod world;

pub mod prelude {
    pub use super::{
        world::{
            World,
            archetype_registry::{
                ArchetypeRegistry,
                archetype_id::{
                    ArchetypeId
                },
                stored_archetype::{
                    StoredArchetype,
                    archetype::{
                        Archetype,
                        type_map_access::{
                            TypeMapAccess
                        },
                        stored_type_map::{
                            StoredTypeMap,
                            type_map::{
                                TypeMap,
                                stored_type::{
                                    StoredType
                                },
                                entity_access::{
                                   EntityAccess
                                },
                                entity_id::{
                                    EntityId
                                },
                            }
                        }
                    }
                },
                archetype_access::{
                    ArchetypeAccess
                }
            },
            query::{
                Query
            }
        },
    };
}