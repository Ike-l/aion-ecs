use std::any::TypeId;

use aion_program::prelude::AutoRegistry;

use crate::prelude::{StoredTypeMap, TypeMapAccess};

pub mod stored_type_map;
pub mod type_map_access;

pub struct Archetype {
    type_map_registry: AutoRegistry<TypeId, StoredTypeMap, TypeMapAccess>
} 