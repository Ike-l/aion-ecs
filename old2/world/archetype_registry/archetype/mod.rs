use std::any::{Any, TypeId};

use aion_state::prelude::Registry;

use crate::prelude::TypeMap;

pub mod type_map;

pub struct Archetype {
    type_map: Registry<TypeId, TypeMap<Box<dyn Any>>>
}